package com.oztz.hackinglabmobile.activity;

import android.app.Activity;
import android.content.Intent;
import android.media.MediaPlayer;
import android.net.Uri;
import android.os.Bundle;
import android.widget.MediaController;
import android.widget.Toast;
import android.widget.VideoView;

import com.google.gson.Gson;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.Challenge;

public class ChallengeDetailActivity extends Activity {

    Challenge challenge;
    VideoView videoView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        challenge = loadChallenge();
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_challenge_detail);

        videoView = (VideoView) findViewById(R.id.challenge_detail_video);
        setupVideo();
    }

    private void setupVideo(){
        if(challenge.aboutchallenge != null && challenge.aboutchallenge.length() > 1){
            Uri videoUrl = Uri.parse(challenge.aboutchallenge + ".webm");
            videoView.setVideoURI(videoUrl);
            MediaController mc = new MediaController(this);
            mc.setAnchorView(videoView);
            videoView.setMediaController(mc);
            videoView.setOnErrorListener(new MediaPlayer.OnErrorListener() {
                @Override
                public boolean onError(MediaPlayer mp, int what, int extra) {
                    String message = "Can not play this video!";
                    if (extra == mp.MEDIA_ERROR_UNSUPPORTED) {
                        message = "Unsupported file format";
                    } else if (extra == mp.MEDIA_ERROR_TIMED_OUT) {
                        message = "No internet connection";
                    }
                    Toast.makeText(getApplicationContext(), message, Toast.LENGTH_SHORT).show();
                    finish();
                    return false;
                }
            });
            videoView.start();
        }
    }

    private Challenge loadChallenge(){
        Intent intent = getIntent();
        return new Gson().fromJson(intent.getStringExtra("challenge"), Challenge.class);
    }
}
