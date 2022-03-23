#include <ArduinoJson.h>
//#include <WiFiClientSecure.h>
#include <WiFi.h>
#include <HTTPClient.h>

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
  tft.fillScreen(TFT_WHITE);

  tft.setCursor(49, 40);
  tft.setTextSize(1);
  tft.setTextColor(TFT_BLACK);

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

    pm25 = (int)last["pm25"];
    readDate = (const char *)last["date"];
  } else {
    Serial.println("Error code: " + httpResponseCode);
  }
  http.end();
}


void loop() {
  getMeasurements();
  writeHeader();
  writeData();
  delay(15000);
}

void writeHeader() {
  tft.setTextColor(TFT_BLACK);

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
  tft.fillScreen(TFT_WHITE);
  tft.setCursor(1, 15 + 11);
  
  tft.setTextSize(1);
  tft.setTextColor(TFT_BLUE);
  if(pm25 >= 13) tft.setTextColor(TFT_GREENYELLOW);
  if(pm25 >= 35) tft.setTextColor(TFT_YELLOW);
  if(pm25 >= 55) tft.setTextColor(TFT_RED);
  tft.println("pm25: ");
  tft.setCursor(30, 15 + 11);
  tft.setTextSize(6);
  tft.println(String(pm25));

  tft.setCursor(30, 80);
  tft.setTextSize(3);
  tft.println(String(pm25*5) + "%");

  tft.setCursor(2, 115);
  tft.setTextSize(1);
  tft.println(readDate);
}
