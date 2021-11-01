# arduino_looko2

## Setup
1. Install new board/hardware:

https://github.com/espressif/arduino-esp32#installation-instructions

2. Use "ESP 32 DEV Module"

3. Add libraries:
```
ArduinoJson
TFT_eSPI
```

4. On Ubuntu/Linux install following packages:
```
sudo apt install python3-serial python-is-python3
```

5. On Ubuntu/Linux you need to ensure you have correct permissions
```
sudo chown <USER> /dev/ttyUSB0
```


## Notes
Parts of this project were inspired by https://github.com/arcbtc/Quickening