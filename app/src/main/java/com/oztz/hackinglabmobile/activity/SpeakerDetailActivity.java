package com.oztz.hackinglabmobile.activity;

import android.content.Intent;
import android.net.Uri;
import android.os.Bundle;
import android.support.v7.app.ActionBar;
import android.support.v7.app.ActionBarActivity;
import android.view.Menu;
import android.view.View;
import android.widget.Button;
import android.widget.ImageView;
import android.widget.LinearLayout;
import android.widget.TextView;

import com.google.gson.Gson;
import com.nostra13.universalimageloader.core.DisplayImageOptions;
import com.nostra13.universalimageloader.core.ImageLoader;
import com.nostra13.universalimageloader.core.ImageLoaderConfiguration;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.EventItem;
import com.oztz.hackinglabmobile.businessclasses.Speaker;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.AuthImageDownloader;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.RequestTask;

import java.util.ArrayList;
import java.util.List;

public class SpeakerDetailActivity extends ActionBarActivity implements HttpResult {

    Speaker speaker;
    TextView title, description;
    ImageView flag, speakerImage;
    ImageLoader imageLoader;
    LinearLayout descriptionLayout;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        speaker = loadSpeaker();
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_speaker_detail);
        title = (TextView) findViewById(R.id.speakerDetail_Title_TextView);
        description = (TextView) findViewById(R.id.speakerDetail_Description_TextView);
        flag = (ImageView) findViewById(R.id.speakerDetail_Flag_ImageView);
        speakerImage = (ImageView) findViewById(R.id.speakerDetail_speakerImage);
        descriptionLayout = (LinearLayout) findViewById(R.id.speakerDetail_descriptionLayout);
        new RequestTask(this).execute(getResources().getString(R.string.rootURL) + "event/" +
                String.valueOf(App.eventId) + "/eventitems", "eventItems");
        SetupView();
    }

    private void SetupView(){
        if(speaker.title != null){
            title.setText(speaker.title + " " + speaker.name);
        } else{
            title.setText(speaker.name);
        }
        flag.setImageURI(Uri.parse("android.resource://com.oztz.hackinglabmobile/drawable/flag_"
                + speaker.nationality.toLowerCase()));
        description.setText(speaker.description);
        if(speaker.media != null && speaker.media.length() > 1){
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
    }

    private Speaker loadSpeaker(){
        Intent intent = getIntent();
        return new Gson().fromJson(intent.getStringExtra("speaker"), Speaker.class);
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
        actionBar.setTitle(getResources().getString(R.string.navigationItem_speaker));
    }

    private EventItem[] getSpeakerItems(EventItem[] items, int speakerID){
        List<EventItem> list = new ArrayList<EventItem>();
        for(int i=0; i<items.length; i++){
            if(items[i].speakerIDFK == speakerID){
                list.add(items[i]);
            }
        }
        return list.toArray(new EventItem[list.size()]);
    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        if(JsonString != null){
            if(requestCode.equals("eventItems"))
            try {
                EventItem[] eventItems = new Gson().fromJson(JsonString, EventItem[].class);
                eventItems = getSpeakerItems(eventItems, speaker.speakerID);

                if(eventItems.length > 0){
                    TextView t = new TextView(getApplicationContext());
                    t.setText(getResources().getString(R.string.speeches_by) + " " +
                            title.getText().toString() + ":");
                    t.setTextColor(description.getCurrentTextColor());
                    descriptionLayout.addView(t);

                    for(int i=0; i<eventItems.length; i++){
                        final EventItem item = eventItems[i];
                        Button b = new Button(getApplicationContext());
                        b.setText(eventItems[i].name);
                        b.setOnClickListener(new View.OnClickListener() {
                            @Override
                            public void onClick(View v) {
                                Intent intent = new Intent(getApplicationContext(), EventItemDetailActivity.class);
                                intent.putExtra("eventItem", new Gson().toJson(item, EventItem.class));
                                intent.putExtra("speaker", new Gson().toJson(speaker));
                                startActivity(intent);
                            }
                        });
                        descriptionLayout.addView(b);
                    }
                }

            } catch(Exception e){}
        }
    }
}
