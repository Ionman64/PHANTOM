package com.oztz.hackinglabmobile.adapter;

import android.content.Context;
import android.net.Uri;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ArrayAdapter;
import android.widget.ImageView;
import android.widget.TextView;

import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.Team;

/**
 * Created by Tobi on 25.03.2015.
 */
public class TeamsAdapter extends ArrayAdapter {

    public TeamsAdapter(Context context, int resource, Team[] teams) {
        super(context, resource, teams);
    }

    @Override
    public View getView(int position, View convertView, ViewGroup parent) {

        View v = convertView;

        if (v == null) {
            LayoutInflater inflater = LayoutInflater.from(getContext());
            v = inflater.inflate(R.layout.item_teams, null);
        }

        Team item = (Team)getItem(position);

        if (item != null) {
            TextView teamName = (TextView) v.findViewById(R.id.teams_team_name);
            ImageView flag = (ImageView) v.findViewById(R.id.teams_team_flag);
            if (teamName != null) {
                teamName.setText(item.groupname);
            }
            if (flag != null) {
                flag.setImageURI(Uri.parse("android.resource://com.oztz.hackinglabmobile/drawable/flag_"
                        + item.nationality.toLowerCase()));
            }
        }
        return v;
    }
}
