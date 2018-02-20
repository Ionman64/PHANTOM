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
import com.oztz.hackinglabmobile.activity.VotingDetailActivity;
import com.oztz.hackinglabmobile.adapter.VotingAdapter;
import com.oztz.hackinglabmobile.businessclasses.Voting;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.RequestTask;

import java.text.ParseException;
import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Calendar;
import java.util.List;
import java.util.Timer;
import java.util.TimerTask;

/**
 * Created by Tobi on 20.03.2015.
 */
public class VotingFragment extends Fragment implements HttpResult {

    private static final String ARG_SECTION_NUMBER = "section_number";
    private ListView votingListView;
    private List<Voting> currentVotings;
    RequestTask rt;
    Timer timer;

    public static VotingFragment newInstance(int sectionNumber) {
        Log.d("DEBUG", "ConferenceFragment.newInstance(" + String.valueOf(sectionNumber) + ")");
        VotingFragment fragment = new VotingFragment();
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
        View view = inflater.inflate(R.layout.fragment_voting, container, false);
        votingListView = (ListView)view.findViewById(R.id.voting_listview);
        currentVotings = new ArrayList<Voting>();
        reloadVotings();
        return view;
    }

    private void reloadVotings(){
        rt = new RequestTask(this);
        rt.execute(getResources().getString(R.string.rootURL) + "" +
                "event/" + String.valueOf(App.eventId) + "/votings", "voting");
    }

    private void startReloadTask(){
        timer = new Timer();
        timer.scheduleAtFixedRate(new TimerTask() {
            @Override
            public void run() {
                reloadVotings();
            }
        }, 0, 5000);
    }

    @Override
    public void onAttach(Activity activity) {
        super.onAttach(activity);
        ((MainActivity) activity).onSectionAttached(getArguments().getInt(
                ARG_SECTION_NUMBER));
    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        if(requestCode.equals("voting")) {
            Voting[] votings = null;

            try {
                votings = new Gson().fromJson(JsonString, Voting[].class);
                currentVotings.clear();
                for (int i = 0; i < votings.length; i++) {
                    if (votings[i].status.equals("voting")) {
                        votings[i].votingEnd = getEndTime(votings[i]);
                        currentVotings.add(votings[i]);
                    }
                }
            } catch (Exception e) {
                Toast.makeText(getActivity(), "Error getting data", Toast.LENGTH_SHORT);
            }
        }
        if(currentVotings.size() > 0){
            votingListView.setAdapter(new VotingAdapter(getActivity(), R.layout.item_voting, currentVotings.toArray(new Voting[currentVotings.size()])));
            votingListView.setOnItemClickListener(new AdapterView.OnItemClickListener() {
                @Override
                public void onItemClick(AdapterView parent, View view, int position, long id) {
                    Intent intent = new Intent(getActivity(), VotingDetailActivity.class);
                    intent.putExtra("voting", new Gson().toJson(currentVotings.toArray(new Voting[currentVotings.size()])[position], Voting.class));
                    startActivity(intent);
                }
            });
        }
    }

    @Override
    public void onPause(){
        super.onPause();
        Log.d("DEBUG", "VotingFragment.onPause()");
        if(rt != null){
            rt.cancel(true);
        }
        if(timer != null){
            timer.cancel();
        }

    }

    @Override
    public void onResume(){
        super.onResume();
        Log.d("DEBUG", "VotingFragment.onResume()");
        startReloadTask();
    }

    private String getEndTime(Voting v) throws ParseException {
        SimpleDateFormat parser = new SimpleDateFormat("HH:mm:ss");
        Calendar duration = Calendar.getInstance();
        Calendar c = Calendar.getInstance();
        duration.setTime(parser.parse(v.votingDuration));
        c.setTime(parser.parse(v.votingStarted));
        c.add(Calendar.HOUR_OF_DAY, duration.get(Calendar.HOUR_OF_DAY));
        c.add(Calendar.MINUTE, duration.get(Calendar.MINUTE));
        c.add(Calendar.SECOND, duration.get(Calendar.SECOND));
        return parser.format(c.getTime());
    }
}
