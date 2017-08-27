fn create_gtk_context(gmaincontext) -> TweetFeedContext {
  let create_gtk_wakeup_signal = |handler| GtkWakeupSignal(gmaincontext, handler);
  create_context(create_gtk_wakeup_signal)
}

fn create_context(wakeupsignal_factory) -> TweetFeedContext {
  let (ui_event_recv, ui_event_send) = channel::<UIEvent>();
  let (ui_signal_recv, ui_signal_send) = wakeupsignal_factory(UISignalHandler::new(ui_event_recv));
  let ui_send = SignalSender::new(ui_signal_send, ui_event_send);
  let event_loop = mio::EventLoop::new().unwrap();
  let domain_sender = event_loop.channel();

  let domain_thread = thread::spawn(move || {
    event_loop.run(DomainHandler::new(ui_send));
  });

  TweetFeedContext {
    domain_thread: domain_thread,
    domain_sender: domain_sender,
    wakeup_recv: ui_signal_recv,
    handles: HandleCreator::new()
  }
}

struct TweetFeedContext {
  JoinHandle<_> domain_thread,
  Sender<DomainEvent> domain_sender,
  WakeupSignalReceiver: wakeup_recv,  //mainly to be able to drop it
  HandleCreator: handles //could use an atomic if we need to create handles from both threads
}

type Handle = u32;

impl TweetFeedContext {
  fn stream_create(&mut self, config: &StreamConfig) -> Handle {
    let h = self.handles.create();
    self.domain_sender.send(DomainEvent::StreamCreate(h, config));
    return h;
  }

  fn stream_start(&mut self, stream: Handle, callback) {
    self.wakeup_recv.handler.set_tweet_callback(callback);
    self.domain_sender.send(DomainEvent::StreamStart(h));
  }

  fn stream_destroy(&mut self, stream: Handle) {
    self.domain_sender.send(DomainEvent::StreamDestroy(h));
  }
}


struct UISignalHandler {
  Receiver<UIEvent> ui_event_rx;
  //c callback here, needs to be updateable from c api
}

impl UISignalHandler {
  fn set_tweet_callback(&mut self, callback) {
    self.tweet_callback = callback;
  }
}

impl UISignalHandler for SignalHandler {
  fn wakeup(&mut self) {
    let evt = self.ui_event_rx.receive();
    self.tweet_callback(evt.tweet);
  }
}

//Domain code running in the domain thread from here

struct DomainHandler {
  TweetFeedDomainRoot root,
  Sender<UIEvent> ui_sender
}

impl Handler for DomainHandler {
  fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Self::Message) {
    if let Some(evt) = self.root.incoming_domain_event(msg) {
      self.ui_sender.send(evt);
    }
  }
}

trait TweetFeedDomainRoot {
  fn incoming_http_message(&mut self, buffer: &u8[]) -> (Option<UIEvent>, Option<IOEvent>);
  fn incoming_domain_event(&mut self, evt: DomainEvent) -> (Option<UIEvent>, Option<IOEvent>);
}
