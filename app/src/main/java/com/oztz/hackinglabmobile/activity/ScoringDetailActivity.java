package com.oztz.hackinglabmobile.activity;

import android.app.ProgressDialog;
import android.content.Intent;
import android.os.Bundle;
import android.support.v7.app.ActionBar;
import android.support.v7.app.ActionBarActivity;
import android.view.Menu;
import android.widget.ListView;
import android.widget.Toast;

import com.google.gson.Gson;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.adapter.ChallengeScoringAdapter;
import com.oztz.hackinglabmobile.businessclasses.Challenge;
import com.oztz.hackinglabmobile.businessclasses.ChallengeScore;
import com.oztz.hackinglabmobile.businessclasses.Team;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.RequestTask;

import java.util.ArrayList;
import java.util.List;

public class ScoringDetailActivity extends ActionBarActivity implements HttpResult {

    ChallengeScore[] scores;
    ListView scoresListView;
    ProgressDialog loading;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_scoring_detail);
        scoresListView = (ListView) findViewById(R.id.Scoring_Detail_List_View);
        new RequestTask(this).execute(getResources().getString(R.string.hackingLabUrl) +
                "SlideService/GetCaseAndGroup/" + String.valueOf(App.eventId), "groupChallenges");
        loading = ProgressDialog.show(this, "Loading", "getting solved challenges...", true, true);

    }

    private ChallengeScore[] getScores(String challengesJson){
        Intent intent = getIntent();
        Team team = new Gson().fromJson(intent.getStringExtra("team"), Team.class);
        Challenge[] challenges = new Gson().fromJson(challengesJson, Challenge[].class);

        List<ChallengeScore> scoreList = new ArrayList<ChallengeScore>();

        for(int i=0; i<challenges.length; i++){
            Challenge c = challenges[i];
            for(int j=0; j<c.groupscore.length; j++){
                Team t = c.groupscore[j];
                if(t.groupID == team.groupID && t.score > 0){
                    scoreList.add(new ChallengeScore(c.modulID, c.title, t.groupID, t.groupname, t.score, c.level));
                }
            }
        }
        return scoreList.toArray(new ChallengeScore[scoreList.size()]);
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        // Inflate the menu; this adds items to the action bar if it is present.
        getMenuInflater().inflate(R.menu.menu_team_detail, menu);
        restoreActionBar();
        return true;
    }

    public void restoreActionBar() {
        ActionBar actionBar = getSupportActionBar();
        actionBar.setNavigationMode(ActionBar.NAVIGATION_MODE_STANDARD);
        actionBar.setDisplayShowTitleEnabled(true);
        actionBar.setTitle(getResources().getString(R.string.solved_challenges));
    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        try
        {
            scores = getScores(JsonString);
            ChallengeScoringAdapter adapter = new ChallengeScoringAdapter(getApplicationContext(), R.layout.item_challenges_scoring, scores);
            scoresListView.setAdapter(adapter);
            adapter.notifyDataSetChanged();
            loading.dismiss();

        } catch(Exception e){
            Toast.makeText(this, "Error getting data", Toast.LENGTH_SHORT);
        }
    }
}
