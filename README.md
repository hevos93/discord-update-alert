# Discord Update Alerter
This repository defines an API that sends a webhook to a specified discord server when an update is registered.

The API works with MongoDB and stores every recorded update for future use.

For reference:
 - 1091500  =>  Cyberpunk 2077
 - 292030   =>  The Witcher 3: Wild Hunt
 - 573090   =>  Stormworks
 - 1144200  =>  Ready or Not

ENV variables: 
 - API_PORT
 - MONGO_URL
 - Discord Webhooks
   - CYBERPUNK2077_HOOK
   - WITCHER3_HOOK
   - STORMWORKS_HOOK
   - READY_OR_NOT_HOOK

MongoDB:
 - Database(discord-rss)
   - Collection (app-id)
     - Document
       - _id: ObjectId
       - title: String
       - link: String
       - pub_date: DateTime
       - img: String