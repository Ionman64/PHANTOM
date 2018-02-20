package com.oztz.hackinglabmobile.adapter;

import android.content.Context;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ArrayAdapter;
import android.widget.TextView;

import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.Voting;

/**
 * Created by Tobi on 25.03.2015.
 */
public class VotingAdapter extends ArrayAdapter {

    public VotingAdapter(Context context, int resource, Voting[] votings) {
        super(context, resource, votings);
    }

    @Override
    public View getView(int position, View convertView, ViewGroup parent) {

        View v = convertView;

        if (v == null) {
            LayoutInflater inflater = LayoutInflater.from(getContext());
            v = inflater.inflate(R.layout.item_voting, null);
        }

        Voting item = (Voting)getItem(position);

        if (item != null) {
            TextView name = (TextView) v.findViewById(R.id.voting_name);
            TextView team = (TextView) v.findViewById(R.id.voting_team_name);
            TextView countdown = (TextView) v.findViewById(R.id.voting_countdown);

            if (name != null) {
                name.setText(item.name);
            }
        }
        return v;
    }
}
