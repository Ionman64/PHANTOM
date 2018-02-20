package com.oztz.hackinglabmobile.adapter;

import android.content.Context;
import android.graphics.Color;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ArrayAdapter;
import android.widget.ImageView;
import android.widget.LinearLayout;
import android.widget.TextView;

import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.Challenge;

/**
 * Created by Tobi on 25.03.2015.
 */
public class ChallengesAdapter extends ArrayAdapter {

    public ChallengesAdapter(Context context, int resource, Challenge[] challenges) {
        super(context, resource, challenges);
    }

    @Override
    public View getView(int position, View convertView, ViewGroup parent) {

        View v = convertView;

        if (v == null) {
            LayoutInflater inflater = LayoutInflater.from(getContext());
            v = inflater.inflate(R.layout.item_challenges, null);
        }

        Challenge item = (Challenge)getItem(position);

        if (item != null) {
            TextView challengeName = (TextView) v.findViewById(R.id.item_challenge_name);
            ImageView symbol = (ImageView) v.findViewById(R.id.item_challenge_symbol);
            LinearLayout difficulty = (LinearLayout) v.findViewById(R.id.item_challenges_difficulty);
            if (challengeName != null) {
                challengeName.setText(item.title);
            }
            switch(item.level){
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
