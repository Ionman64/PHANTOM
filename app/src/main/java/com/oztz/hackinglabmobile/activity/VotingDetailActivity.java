package com.oztz.hackinglabmobile.activity;

import android.app.ProgressDialog;
import android.content.Intent;
import android.graphics.Color;
import android.os.Bundle;
import android.os.CountDownTimer;
import android.support.v7.app.ActionBar;
import android.support.v7.app.ActionBarActivity;
import android.util.Log;
import android.view.Gravity;
import android.view.Menu;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.SeekBar;
import android.widget.TextView;
import android.widget.Toast;

import com.google.gson.Gson;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.Slider;
import com.oztz.hackinglabmobile.businessclasses.Vote;
import com.oztz.hackinglabmobile.businessclasses.Voting;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.PostTask;
import com.oztz.hackinglabmobile.helper.RequestTask;

import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Calendar;
import java.util.Date;
import java.util.List;
import java.util.TimeZone;

public class VotingDetailActivity extends ActionBarActivity implements HttpResult {

    Voting voting;
    String serverTime, qrCode;
    TextView title, countDown;
    List<TextView> labels;
    List<SeekBar> scrollBars;
    Button voteButton;
    LinearLayout scrollBarHolder;
    boolean isJury;
    long diff;
    ProgressDialog loading;
    int postCount = 0;
    boolean successfulVoting = true;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        App.loadVariables();
        voting = loadVoting();
        voting.votingEnd = getEndTime();
        checkForJury();
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_voting_detail);
        scrollBarHolder = (LinearLayout) findViewById(R.id.voting_detail_scrollbar_holder);
        new RequestTask(this).execute(getResources().getString(R.string.rootURL) + "voting/" + voting.votingID + "/sliders", "sliders");
        new RequestTask(this).execute(getResources().getString(R.string.rootURL) + "time", "time");
        title = (TextView) findViewById(R.id.voting_detail_votingName);
        countDown = (TextView) findViewById(R.id.voting_countdown);
        voteButton = (Button) findViewById(R.id.voting_button_vote);
        voteButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                postVoting();
            }
        });

        labels = new ArrayList<TextView>();
        scrollBars = new ArrayList<SeekBar>();
        loadCountdown();
    }

    private long getTimeDiff(){
        String[] startParts = serverTime.split(":");
        String[] endParts = voting.votingEnd.split(":");
        if(startParts.length == 3 && endParts.length == 3) {
            long startMillis = Integer.parseInt(startParts[0]) * 3600000 +
                    Integer.parseInt(startParts[1]) * 60000 +
                    Integer.parseInt(startParts[2]) * 1000;
            long endMillis = Integer.parseInt(endParts[0]) * 3600000 +
                    Integer.parseInt(endParts[1]) * 60000 +
                    Integer.parseInt(endParts[2]) * 1000;
            return endMillis - startMillis;
        }
        return 0;
    }

    private void checkForJury(){
        qrCode = App.db.getQrCode("jury", App.eventId);
        isJury = qrCode != null;
    }

    private String getEndTime() {
        try {
            SimpleDateFormat parser = new SimpleDateFormat("HH:mm:ss");
            Calendar duration = Calendar.getInstance();
            Calendar c = Calendar.getInstance();
            duration.setTime(parser.parse(voting.votingDuration));
            c.setTime(parser.parse(voting.votingStarted));
            c.add(Calendar.HOUR_OF_DAY, duration.get(Calendar.HOUR_OF_DAY));
            c.add(Calendar.MINUTE, duration.get(Calendar.MINUTE));
            c.add(Calendar.SECOND, duration.get(Calendar.SECOND));
            return parser.format(c.getTime());
        } catch (Exception e){
            Log.d("DEBUG", e.getMessage());
        }
        return "";
    }

    private String getTimeString(long diff) {
        Date date = new Date(diff);
        SimpleDateFormat sdf = new SimpleDateFormat("HH:mm:ss");
        sdf.setTimeZone(TimeZone.getTimeZone("GMT"));
        return sdf.format(date);
    }

    private void loadCountdown(){
        try
        {
            diff = getTimeDiff();
            if(diff > 0){
                new CountDownTimer(diff, 1000) {
                    public void onTick(long millisUntilFinished) {
                        countDown.setText(getTimeString(millisUntilFinished));
                    }
                    public void onFinish() {
                        countDown.setText("--:--:--");
                        if(! isJury){
                            Toast.makeText(getApplicationContext(), "Time is over!", Toast.LENGTH_SHORT).show();
                            voteButton.setEnabled(false);
                        }
                    }
                }.start();
            } else if(!isJury){
                voteButton.setEnabled(false);
                Toast.makeText(getApplicationContext(), "Time is over!", Toast.LENGTH_SHORT).show();
            }
        } catch (Exception e){

        }
    }

    private void closeVoting(){
        this.finish();
    }

    private void postVoting(){
        loading = ProgressDialog.show(this, "Loading", "Send voting...", true, true);
        for(int i=0; i<scrollBars.size(); i++){
            SeekBar s = scrollBars.get(i);
            if(isJury){ //Ist ein Jurymitglied
                Vote v = new Vote(0, s.getProgress(), false, (int)s.getTag(), App.userId);
                String json = new Gson().toJson(v, Vote.class);
                new PostTask(this).execute(getResources().getString(R.string.rootURL) + "vote", json, qrCode);
            } else { //Ist kein Jurymitglied
                Vote v = new Vote(0, s.getProgress(), false, (int)s.getTag(), App.userId);
                String json = new Gson().toJson(v, Vote.class);
                new PostTask(this).execute(getResources().getString(R.string.rootURL) + "vote", json);
            }
        }
    }

    private void SetupView(Slider[] sliders){
        title.setText(voting.name);
        for(int i=0; i<sliders.length; i++){
            final int index = i;

            SeekBar s = new SeekBar(getApplicationContext());
            s.setMax(voting.sliderMaxValue);
            s.setIndeterminate(false);
            s.setLayoutParams(new LinearLayout.LayoutParams(ViewGroup.LayoutParams.WRAP_CONTENT, ViewGroup.LayoutParams.WRAP_CONTENT, 1.0f));
            s.setProgress(voting.sliderMaxValue / 2);
            s.setOnSeekBarChangeListener(new SeekBar.OnSeekBarChangeListener() {
                @Override
                public void onProgressChanged(SeekBar seekBar, int progress, boolean fromUser) {
                    labels.get(index).setText(String.valueOf(progress) + "/" + String.valueOf(voting.sliderMaxValue));
                }

                @Override
                public void onStartTrackingTouch(SeekBar seekBar) {

                }

                @Override
                public void onStopTrackingTouch(SeekBar seekBar) {

                }
            });
            s.setTag(sliders[i].sliderID);
            scrollBars.add(s);

            TextView scrollBarValue = new TextView(getApplicationContext());
            scrollBarValue.setLayoutParams(new LinearLayout.LayoutParams(ViewGroup.LayoutParams.WRAP_CONTENT, ViewGroup.LayoutParams.WRAP_CONTENT));
            scrollBarValue.setText(String.valueOf(voting.sliderMaxValue / 2) + "/" + String.valueOf(voting.sliderMaxValue));
            scrollBarValue.setTextColor(Color.BLACK);
            labels.add(scrollBarValue);

            LinearLayout layout = new LinearLayout(getApplicationContext());
            layout.setLayoutParams(new LinearLayout.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.WRAP_CONTENT));
            layout.setGravity(Gravity.CENTER_VERTICAL);
            layout.setOrientation(LinearLayout.HORIZONTAL);
            layout.addView(s);
            layout.addView(scrollBarValue);

            TextView sliderTitle = new TextView(getApplicationContext());
            sliderTitle.setTextAppearance(getApplicationContext(), R.style.Base_TextAppearance_AppCompat_Large);
            LinearLayout.LayoutParams lp = new LinearLayout.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.WRAP_CONTENT);
            lp.setMargins(0, 20, 0, 0);
            sliderTitle.setLayoutParams(lp);
            sliderTitle.setTextColor(Color.BLACK);
            sliderTitle.setText(sliders[i].name);

            scrollBarHolder.addView(sliderTitle);
            scrollBarHolder.addView(layout);
        }
    }

    private Voting loadVoting(){
        Intent intent = getIntent();
        return new Gson().fromJson(intent.getStringExtra("voting"), Voting.class);
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        // Inflate the menu; this adds items to the action bar if it is present.
        getMenuInflater().inflate(R.menu.menu_voting_detail, menu);
        restoreActionBar();
        return true;
    }

    public void restoreActionBar() {
        ActionBar actionBar = getSupportActionBar();
        actionBar.setNavigationMode(ActionBar.NAVIGATION_MODE_STANDARD);
        actionBar.setDisplayShowTitleEnabled(true);
        actionBar.setTitle(getResources().getString(R.string.navigationItem_voting));

    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        try {
            if(requestCode.equals("sliders")) {
                Slider[] sliders = new Gson().fromJson(JsonString, Slider[].class);
                //Slider[] sliders = {new Slider(1, 1, 2, "Language"), new Slider(2, 1, 1, "Graphics"), new Slider(3, 1, 3, "Solution")};
                SetupView(sliders);
            } else if(requestCode.equals("time")) {
                if (JsonString.contains(".")) {
                    serverTime = JsonString.substring(0, JsonString.indexOf("."));
                } else {
                    serverTime = JsonString;
                }
                loadCountdown();
            } else if(requestCode.equals("POST")){
                Log.d("DEBUG", JsonString);
                if(JsonString.contains("HTTP Status 423")){
                    successfulVoting = false;
                }
                postCount++;
            }

            if(postCount >= scrollBars.size()){
                loading.dismiss();
                if(successfulVoting){
                    Toast.makeText(getApplicationContext(), "Thank you for your voting!",
                            Toast.LENGTH_SHORT).show();
                } else {
                    Toast.makeText(getApplicationContext(), "You have already voted!",
                            Toast.LENGTH_SHORT).show();
                }
                this.finish();
            }

        } catch(Exception e){
            Toast.makeText(this, "Error getting data", Toast.LENGTH_SHORT);
        }
    }
}
