#include <stdlib.h>

typedef uint32_t TweetFeedHandle;
typedef struct {} TweetFeedContext;

#ifdef PLATFORM_GTK
  #include <gtk/gtk.h>
  TweetFeedContext* tweetfeed_context_new_for_gtk(GMainContext* ui_ctx);
#endif

#ifdef PLATFORM_LIBDISPATCH
  #include <dispatch/dispatch.h>
  TweetFeedContext* tweetfeed_context_new_for_libdispatch(dispatch_queue_t ui_queue);
#endif

void tweetfeed_context_destroy(TweetFeedContext* ctx);

typedef struct {
  ByteSlice consumer_key;
  ByteSlice consumer_secret;
  ByteSlice token;
  ByteSlice token_secret;
} TweetFeedConfig;

typedef TweetFeedHandle TweetFeedStreamHandle;

typedef struct {
  const uint8_t *bytes;
  size_t length;
} ByteSlice;

typedef struct {
  ByteSlice user_name;
  ByteSlice body;
} Tweet;

TweetFeedStreamHandle tweetfeed_stream_new(TweetFeedContext* ctx, const TweetFeedConfig* cfg);
void tweetfeed_stream_start(TweetFeedStreamHandle stream, void(TweetCallback*)(Tweet* tweet));
void tweetfeed_tweet_free(Tweet* tweet);
