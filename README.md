# Discord Update Alerter
This repository defines an API that sends a webhook to a specified discord server when an update is registered.

The API works with MongoDB and stores every update there.




Inside src:

- api is for modularizing api handlers
- models is for modularizing data logics
- repositories is for modularizing databases (and in this case other API requests)