//#include <ArduinoHttpClient.h>
#include <ArduinoJson.h>
#include <WiFiClientSecure.h>
#include <TFT_eSPI.h>
#include "secrets.h"

TFT_eSPI tft = TFT_eSPI(); 
String wifiStatus = "NOT CONNECTED";

String readDate = "";
int pm25 = 0;

void setup() {
  tft.begin();
  tft.setRotation(1);
  Serial.begin(115200);
  tft.fillScreen(TFT_BLACK);

  tft.setCursor(49, 40);
  tft.setTextSize(1);
  tft.setTextColor(TFT_WHITE);

  WiFi.begin(SSID, PASSWORD);   

  int i = 0;
  while (WiFi.status() != WL_CONNECTED) {
    if(i >= 5){
     tft.fillScreen(TFT_BLACK);
     tft.setCursor(55, 20);
     tft.setTextSize(1);
     tft.setTextColor(TFT_RED);
     tft.println("WIFI NOT CONNECTED");
     Serial.println("WIFI NOT CONNECTED");
     ESP.restart();
    }
    delay(1000);
    i++;
  }
  
  //tft.println("TEST");
  Serial.println("Setup done");
}

void getMeasurements() {
  WiFiClient client; //if using server with SSL switch to WiFiClientSecure
  if (!client.connect(SERVER_ADDR, SERVER_PORT)) {
    Serial.println("unable to connect,  SERVER_ADDR, SERVER_PORT: " + String(SERVER_ADDR) + ", " + SERVER_PORT);
    return;
  }
  String url = "http://"+ String(SERVER_ADDR) + ":" + SERVER_PORT + "/station/" +STATION_ID;
  Serial.println("url: " + url);

  client.print(String("GET ") + url + " HTTP/1.1\r\n" +
               "Host: "+SERVER_ADDR+"\r\n" +
               "User-Agent: ESP32\r\n" +
               "Connection: close\r\n\r\n");
  while (client.connected()) {
    String line = client.readStringUntil('\n');
    Serial.println("line: " +line);
    if (line.startsWith("{")) {
    
      const size_t capacity = 5096;
      DynamicJsonDocument doc(capacity);
      deserializeJson(doc, line);
      int elements = doc["air_quality"].size();
      Serial.println("total elements: " + elements);
      readDate = (const char *)doc["air_quality"][elements-1]["date"];
      pm25 = doc["air_quality"][elements-1]["pm25"];
 
      return;
    }
  }
}

void loop() {
  getMeasurements();
  writeHeader();
  writeData();
  delay(15000);
}

void writeHeader() {
  if (WiFi.status() == WL_CONNECTED)
  {
    wifiStatus = "OK";
  }
  else
  {
    wifiStatus = "ERROR";
  }

  tft.setCursor(43, 0);
  tft.println("WiFi: " + wifiStatus);
}

void writeData() {
  writeLineToTft(1, "pm25: " + pm25);
  writeLineToTft(2, "date: " + readDate);
}

void writeLineToTft(int number, String text) {
  tft.setCursor(1, 15 + 11 * number);
  tft.println(text);
  Serial.println(text);
}
