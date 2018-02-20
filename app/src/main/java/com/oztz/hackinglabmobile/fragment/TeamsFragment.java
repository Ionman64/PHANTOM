package com.oztz.hackinglabmobile.fragment;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.support.v4.app.Fragment;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.AdapterView;
import android.widget.ListView;
import android.widget.Toast;

import com.google.gson.Gson;
import com.oztz.hackinglabmobile.MainActivity;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.activity.TeamDetailActivity;
import com.oztz.hackinglabmobile.adapter.TeamsAdapter;
import com.oztz.hackinglabmobile.businessclasses.Team;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.RequestTask;

/**
 * Created by Tobi on 20.03.2015.
 */
public class TeamsFragment extends Fragment implements HttpResult {

    private static final String ARG_SECTION_NUMBER = "section_number";
    private ListView teamsListView;

    public static TeamsFragment newInstance(int sectionNumber) {
        Log.d("DEBUG", "TeamsFragment.newInstance(" + String.valueOf(sectionNumber) + ")");
        TeamsFragment fragment = new TeamsFragment();
        Bundle args = new Bundle();
        args.putInt(ARG_SECTION_NUMBER, sectionNumber);
        fragment.setArguments(args);
        return fragment;
    }

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
    }

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container,
                             Bundle savedInstanceState)
    {
        View view = inflater.inflate(R.layout.fragment_teams, container, false);
        teamsListView = (ListView) view.findViewById(R.id.teams_listview);

        String url = getResources().getString(R.string.hackingLabUrl) +
                "WebService/GetGroups/" + String.valueOf(App.eventId);
        updateView(App.db.getContentFromDataBase(url));
        new RequestTask(this).execute(url, "teams");

        return view;
    }

    @Override
    public void onAttach(Activity activity) {
        super.onAttach(activity);
        ((MainActivity) activity).onSectionAttached(getArguments().getInt(
                ARG_SECTION_NUMBER));
    }

    private void updateView(String JsonString){
        try {
            final Team[] teams = new Gson().fromJson(JsonString, Team[].class);
            teamsListView.setAdapter(new TeamsAdapter(getActivity(), R.layout.item_teams, teams));
            teamsListView.setOnItemClickListener(new AdapterView.OnItemClickListener() {
                @Override
                public void onItemClick(AdapterView parent, View view, int position, long id) {
                    Intent intent = new Intent(getActivity(), TeamDetailActivity.class);
                    intent.putExtra("team", new Gson().toJson(teams[position], Team.class));
                    startActivity(intent);
                }
            });
        } catch(Exception e){
            Toast.makeText(getActivity(), "Error getting data", Toast.LENGTH_SHORT);
        }
    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        if(requestCode.equals("teams")){
            updateView(JsonString);
        }

    }
}
