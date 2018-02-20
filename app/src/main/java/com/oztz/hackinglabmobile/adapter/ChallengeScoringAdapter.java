package com.oztz.hackinglabmobile.adapter;

import android.content.Context;
import android.graphics.Color;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ArrayAdapter;
import android.widget.LinearLayout;
import android.widget.TextView;

import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.ChallengeScore;

/**
 * Created by Tobi on 25.03.2015.
 */
public class ChallengeScoringAdapter extends ArrayAdapter {

    public ChallengeScoringAdapter(Context context, int resource, ChallengeScore[] challenges) {
        super(context, resource, challenges);
    }

    @Override
    public View getView(int position, View convertView, ViewGroup parent) {
        View v = convertView;

        if (v == null) {
            LayoutInflater inflater = LayoutInflater.from(getContext());
            v = inflater.inflate(R.layout.item_challenges_scoring, null);
        }

        ChallengeScore item = (ChallengeScore)getItem(position);

        if (item != null) {
            TextView challengeName = (TextView) v.findViewById(R.id.challenge_scoring_name_textview);
            TextView score = (TextView) v.findViewById(R.id.challenge_scoring_score_textview);
            LinearLayout difficulty = (LinearLayout) v.findViewById(R.id.challenge_scoring_difficulty);
            if (challengeName != null) {
                challengeName.setText(item.ChallengeName);
            }
            if(score != null){
                score.setText(String.valueOf(item.Score));
            }
            switch(item.ChallengeLevel){
                case 1:
                    difficulty.setBackgroundColor(Color.parseColor("#FF70981F"));
                    break;
                case 2:
                    difficulty.setBackgroundColor(Color.parseColor("#FFFFCC00"));
                    break;
                case 3:
                    difficulty.setBackgroundColor(Color.parseColor("#FFA32121"));
                    break;
            }


        }
        return v;
    }
}
