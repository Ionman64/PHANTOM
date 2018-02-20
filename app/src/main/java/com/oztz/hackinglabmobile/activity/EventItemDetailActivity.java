package com.oztz.hackinglabmobile.activity;

import android.content.Intent;
import android.os.Bundle;
import android.support.v7.app.ActionBar;
import android.support.v7.app.ActionBarActivity;
import android.view.Menu;
import android.widget.ImageView;
import android.widget.TextView;

import com.google.gson.Gson;
import com.nostra13.universalimageloader.core.DisplayImageOptions;
import com.nostra13.universalimageloader.core.ImageLoader;
import com.nostra13.universalimageloader.core.ImageLoaderConfiguration;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.EventItem;
import com.oztz.hackinglabmobile.businessclasses.EventRoom;
import com.oztz.hackinglabmobile.businessclasses.Speaker;
import com.oztz.hackinglabmobile.helper.AuthImageDownloader;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.RequestTask;

public class EventItemDetailActivity extends ActionBarActivity implements HttpResult {

    EventItem eventItem;
    EventRoom eventRoom;
    Speaker speaker;
    TextView titleTextView, dateTextView, timeTextView, roomTextView, speakerTextView, descriptionTextView;
    ImageView speakerImage;
    ImageLoader imageLoader;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_eventitem_detail);
        titleTextView = (TextView) findViewById(R.id.eventItemDetail_Title_TextView);
        descriptionTextView = (TextView) findViewById(R.id.eventItemDetail_description);
        speakerTextView = (TextView) findViewById(R.id.eventItemDetail_speaker);
        dateTextView = (TextView) findViewById(R.id.eventItemDetail_date);
        timeTextView = (TextView) findViewById(R.id.eventItemDetail_time);
        roomTextView = (TextView) findViewById(R.id.eventItemDetail_room);
        speakerImage = (ImageView) findViewById(R.id.eventItemDetail_speakerImage);
        loadArguments();
        SetupView();
    }

    private void SetupView(){
        if(eventItem != null){
            showEventItemInfo();
        }
        if(speaker != null){
            showSpeakerInfo();
        }
        if(eventRoom != null){
            roomTextView.setText(eventRoom.name);
        }
    }

    private void showEventItemInfo(){
        titleTextView.setText(eventItem.name);
        dateTextView.setText(eventItem.date);
        timeTextView.setText(eventItem.startTime + " - " + eventItem.endTime);
        descriptionTextView.setText(eventItem.description);
    }

    private void showSpeakerInfo(){
        if(speaker.media != null && speaker.media.length() > 1) {
            imageLoader = ImageLoader.getInstance();
            ImageLoaderConfiguration config = new ImageLoaderConfiguration.Builder(getApplicationContext())
                    .imageDownloader(new AuthImageDownloader(getApplicationContext(), 5000, 20000))
                    .diskCacheFileCount(50)
                    .defaultDisplayImageOptions(new DisplayImageOptions.Builder()
                            .cacheInMemory(true)
                            .cacheOnDisk(true).build())
                    .build();
            imageLoader.init(config);
            imageLoader.displayImage(speaker.media, speakerImage);
        }
        if(speaker.title != null){
            speakerTextView.setText(speaker.title + " " + speaker.name);
        } else{
            speakerTextView.setText(speaker.name);
        }
    }

    private void loadArguments(){
        Intent intent = getIntent();
        String eventItemJson = intent.getStringExtra("eventItem");
        String speakerJson = intent.getStringExtra("speaker");
        String roomJson = intent.getStringExtra("eventRoom");
        if(eventItemJson != null && eventItemJson.length() > 1){
            eventItem = new Gson().fromJson(eventItemJson, EventItem.class);

            if(speakerJson != null && speakerJson.length() > 1){
                speaker = new Gson().fromJson(speakerJson, Speaker.class);
            } else {
                new RequestTask(this).execute(getResources().getString(R.string.rootURL) + "speaker/" +
                        String.valueOf(eventItem.speakerIDFK), "speaker");
            }

            if(roomJson != null && roomJson.length() > 1){
                eventRoom = new Gson().fromJson(roomJson, EventRoom.class);
            } else {
                new RequestTask(this).execute(getResources().getString(R.string.rootURL) + "eventroom/" +
                        String.valueOf(eventItem.roomIDFK), "room");
            }
        }


    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        // Inflate the menu; this adds items to the action bar if it is present.
        getMenuInflater().inflate(R.menu.menu_speaker_detail, menu);
        restoreActionBar();
        return true;
    }

    public void restoreActionBar() {
        ActionBar actionBar = getSupportActionBar();
        actionBar.setNavigationMode(ActionBar.NAVIGATION_MODE_STANDARD);
        actionBar.setDisplayShowTitleEnabled(true);
        actionBar.setTitle(getResources().getString(R.string.event));
    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        if(requestCode.equals("speaker")){
            try{
                speaker = new Gson().fromJson(JsonString, Speaker.class);
                showSpeakerInfo();
            }catch(Exception e){}
        } else if(requestCode.equals("room")){
            try{
                eventRoom = new Gson().fromJson(JsonString, EventRoom.class);
                roomTextView.setText(eventRoom.name);
            }catch(Exception e){}
        }
    }
}
