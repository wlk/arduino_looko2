# arduino_client

## Setup
1. Install new board/hardware:

https://github.com/espressif/arduino-esp32#installation-instructions

2. Use "ESP 32 DEV Module"

3. Add libraries:
```
ArduinoJson - version 6.18.5
TFT_eSPI - use version from `library` folder
```

4. On Ubuntu/Linux install following packages:
```
sudo apt install python3-serial python-is-python3
```

5. On Ubuntu/Linux you need to ensure you have correct permissions
```
sudo chown <USER> /dev/ttyUSB0
```

## Secrets

There are 3 secrets required, put them into `secrets.sh` with following content:

```
#define SSID "ssid"
#define PASSWORD "pass"
#define STATION_ID "station_id"
#define SERVER_ADDR "ip_address"
#define SERVER_PORT 1337
```


## Notes
Parts of this project were inspired by https://github.com/arcbtc/Quickening