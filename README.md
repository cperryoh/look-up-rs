# look-up-rs
A Tool to spot interesting planes flying overhead. Notifications delivered via home assistant.
## Env file 
| Key            | Desc                      |
|----------------|---------------------------|
| HASSIO_API_KEY | Api key to home assistant |
| HASSIO_URL     | Home assistant url        |
|                |                           |
## config.toml
| Key                       | Desc                                                                                                           |
|---------------------------|----------------------------------------------------------------------------------------------------------------|
| Distance                  | Max distance away for an aircraft                                                                              |
| notify_entry              | Home assisant device to  notify(example: mobile_app_sm_s937u,  see HA Dev Tools>Actions)                       |
| min_height                | Minimum height, this is to avoid things  that are not flying but are being detected                            |
| aircraft_types            | List of aircraft types to filter for(Ex:C17,UH60,etc)                                                          |
| location_entity(optional) | Location entity to use to pull location origin to search for aircraft at, if not present, use static_location  |
| static_location.lat       | Latitude of static location                                                                                    |
| static_location.lon       | Longitude of static location                                                                                   |
| update_interval_min       | Interval to check for new aircraft                                                                             |
