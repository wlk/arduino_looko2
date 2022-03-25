#include <ArduinoJson.h>
#include <WiFi.h>
#include <HTTPClient.h>

#include <TFT_eSPI.h>
#include "secrets.h"

TFT_eSPI tft = TFT_eSPI(); 
String wifiStatus = "NOT CONNECTED";

String readDate = "";
String oldReadDate = "";
int pm25 = 0;
int oldPm25 = 0;

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
  
  Serial.println("Setup done");
}

void getMeasurements() {
  HTTPClient http;
  String url = "http://"+ String(SERVER_ADDR) + ":" + SERVER_PORT + "/station/" +STATION_ID;
  Serial.println("url: " + url);
  http.begin(url);
  int httpResponseCode = http.GET();
  if (httpResponseCode>0) {
    Serial.print("HTTP Response code: ");
    Serial.println(httpResponseCode);
    String payload = http.getString();
    //Serial.println(payload);
    const size_t capacity = 6096;
    DynamicJsonDocument doc(capacity);
    deserializeJson(doc, payload);
    JsonArray elements = doc["air_quality"].as<JsonArray>();
    int total = elements.size();
    JsonObject last = elements[total-1];

    oldPm25 = pm25;
    oldReadDate = readDate;
    pm25 = (int)last["pm25"];
    readDate = (const char *)last["date"];
  } else {
    Serial.println("Error code: " + httpResponseCode);
  }
  http.end();
}


void loop() {
  getMeasurements();
  if(pm25 != oldPm25){
    writeHeader();
    writeData(true);
    writeData(false);
  }
  delay(15000);
}

void writeHeader() {
  tft.setTextColor(TFT_BLACK);

  if (WiFi.status() == WL_CONNECTED) {
    wifiStatus = "OK";
  } else {
    wifiStatus = "ERROR";
  }

  tft.setCursor(43, 0);
  tft.println("WiFi: " + wifiStatus);
}

void writeData(boolean colorOverride) {
  int toWritePm25 = 0;
  String toWriteDate = "";

  tft.setCursor(1, 15 + 11);
  
  tft.setTextSize(1);
  tft.setTextColor(TFT_BLUE);
  if(pm25 >= 13) tft.setTextColor(TFT_GREEN);
  if(pm25 >= 35) tft.setTextColor(TFT_YELLOW);
  if(pm25 >= 55) tft.setTextColor(TFT_RED);
  
  if(colorOverride) {
    tft.setTextColor(TFT_BLACK);
    toWritePm25 = oldPm25;
    toWriteDate = oldReadDate;
  } else {
    toWritePm25 = pm25;
    toWriteDate = readDate;
  }
    
  tft.println("pm25: ");
  tft.setCursor(30, 15 + 11);
  tft.setTextSize(6);
  tft.println(String(toWritePm25));

  tft.setCursor(30, 80);
  tft.setTextSize(3);
  tft.println(String(toWritePm25*5) + "%");

  tft.setCursor(2, 112);
  tft.setTextSize(2);
  toWriteDate.remove(0,11);
  tft.println(toWriteDate);
}
