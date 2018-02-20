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
import com.oztz.hackinglabmobile.activity.ChallengeDetailActivity;
import com.oztz.hackinglabmobile.adapter.ChallengesAdapter;
import com.oztz.hackinglabmobile.businessclasses.Challenge;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.RequestTask;

/**
 * Created by Tobi on 20.03.2015.
 */
public class ChallengesFragment extends Fragment implements HttpResult {

    private static final String ARG_SECTION_NUMBER = "section_number";
    private ListView challengesListView;

    public static ChallengesFragment newInstance(int sectionNumber) {
        Log.d("DEBUG", "ChallengesFragment.newInstance(" + String.valueOf(sectionNumber) + ")");
        ChallengesFragment fragment = new ChallengesFragment();
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
        View view = inflater.inflate(R.layout.fragment_challenges, container, false);
        challengesListView = (ListView)view.findViewById(R.id.challenges_listview);

        String url = getResources().getString(R.string.hackingLabUrl) +
                "SlideService/GetChallenge/" + String.valueOf(App.eventId);
        updateView(App.db.getContentFromDataBase(url));
        new RequestTask(this).execute(url, "Challenge");

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
            final Challenge[] challenges = new Gson().fromJson(JsonString, Challenge[].class);
            challengesListView.setAdapter(new ChallengesAdapter(getActivity(), R.layout.item_challenges, challenges));
            challengesListView.setOnItemClickListener(new AdapterView.OnItemClickListener() {
                @Override
                public void onItemClick(AdapterView parent, View view, int position, long id) {
                    Intent intent = new Intent(getActivity(), ChallengeDetailActivity.class);
                    if(challenges[position].aboutchallenge != null) {
                        intent.putExtra("challenge", new Gson().toJson(challenges[position], Challenge.class));
                        startActivity(intent);
                    } else {
                        Toast.makeText(getActivity().getApplicationContext(),
                                getResources().getString(R.string.error_no_challenge_description),
                                Toast.LENGTH_SHORT).show();
                    }
                }
            });
        } catch(Exception e){
            Toast.makeText(getActivity(), "Error getting data", Toast.LENGTH_SHORT);
        }
    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        if(requestCode.equals("Challenge")){
            updateView(JsonString);
        }

    }
}


