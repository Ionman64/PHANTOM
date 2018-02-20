package com.oztz.hackinglabmobile.fragment;

import android.app.Activity;
import android.graphics.Color;
import android.os.Bundle;
import android.support.v4.app.Fragment;
import android.support.v4.app.FragmentTabHost;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Toast;

import com.google.gson.Gson;
import com.oztz.hackinglabmobile.MainActivity;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.EventItem;
import com.oztz.hackinglabmobile.businessclasses.EventRoom;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.RequestTask;

import java.util.ArrayList;
import java.util.List;

/**
 * Created by Tobi on 20.03.2015.
 */
public class AgendaTabHolderFragment extends Fragment implements HttpResult {

    private static final String ARG_SECTION_NUMBER = "section_number";
    private static final int[] roomColors = {Color.BLUE, Color.GREEN, Color.RED, Color.YELLOW, Color.CYAN};
    private FragmentTabHost mTabHost;
    private String roomsJson, itemsJson;

    public static AgendaTabHolderFragment newInstance(int sectionNumber) {
        Log.d("DEBUG", "AgendaTabHolderFragment.newInstance(" + String.valueOf(sectionNumber) + ")");
        AgendaTabHolderFragment fragment = new AgendaTabHolderFragment();
        Bundle args = new Bundle();
        args.putInt(ARG_SECTION_NUMBER, sectionNumber);
        fragment.setArguments(args);
        return fragment;
    }

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container, Bundle savedInstanceState) {
        mTabHost = new FragmentTabHost(getActivity());
        mTabHost.setup(getActivity(), getChildFragmentManager(), R.layout.fragment_main);

        String urlRooms = getResources().getString(R.string.rootURL) + "event/" +
                String.valueOf(App.eventId) + "/eventrooms";
        String urlItems = getResources().getString(R.string.rootURL) + "event/" +
                String.valueOf(App.eventId) + "/eventitems";


        Log.d("DEBUG", "Load Cached Content...");
        //Load Cached content
        String roomsJsonCached = App.db.getContentFromDataBase(urlRooms);
        String itemsJsonCached = App.db.getContentFromDataBase(urlItems);
        updateView(roomsJsonCached, itemsJsonCached, "cache");

        Log.d("DEBUG", "Loaded Cached Content!");

        //Load online content
        Log.d("DEBUG", "Load Online Content...");
        new RequestTask(this).execute(urlRooms, "eventRooms");
        new RequestTask(this).execute(urlItems, "eventItems");

        return mTabHost;
    }

    @Override
    public void onDestroyView() {
        super.onDestroyView();
        mTabHost = null;
    }

    @Override
    public void onAttach(Activity activity) {
        super.onAttach(activity);
        ((MainActivity) activity).onSectionAttached(getArguments().getInt(
                ARG_SECTION_NUMBER));
    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        if(requestCode.equals("eventRooms")){
            roomsJson = JsonString;
        }
        if(requestCode.equals("eventItems")){
            itemsJson = JsonString;
        }
        if(roomsJson != null && itemsJson != null){
            updateView(roomsJson, itemsJson, "online");
        }
    }

    private EventItem[] getRoomItems(EventItem[] items, int roomID){
        List<EventItem> list = new ArrayList<EventItem>();
        for(int i=0; i<items.length; i++){
            if(items[i].roomIDFK == roomID){
                list.add(items[i]);
            }
        }
        return list.toArray(new EventItem[list.size()]);
    }

    private void updateView(String roomsString, String itemsString, String source){
        if(roomsString != null && itemsString != null){
            try {
                EventRoom[] rooms = new Gson().fromJson(roomsString, EventRoom[].class);
                EventItem[] items = new Gson().fromJson(itemsString, EventItem[].class);
                mTabHost.clearAllTabs();
                mTabHost.invalidate();
                //Load Overview
                Bundle overviewArgs = new Bundle();
                overviewArgs.putString("eventitems", itemsString);
                overviewArgs.putString("rooms", new Gson().toJson(rooms));
                mTabHost.addTab(mTabHost.newTabSpec("Tab1_" + source).setIndicator("All"),
                        AgendaFragment.class, overviewArgs);

                //Load room Views if there is more than one room
                if(rooms.length > 1) {
                    for (int i = 0; i < rooms.length; i++) {
                        EventItem[] roomItems = getRoomItems(items, rooms[i].eventRoomID);
                        String jsonItems = new Gson().toJson(roomItems);
                        Bundle args = new Bundle();
                        args.putString("eventitems", jsonItems);
                        args.putString("rooms", new Gson().toJson(rooms));
                        mTabHost.addTab(mTabHost.newTabSpec("Tab" + String.valueOf(i+2) + "_" + source).setIndicator(rooms[i].name),
                                AgendaFragment.class, args);
                    }
                }
            } catch(Exception e){
                Toast.makeText(getActivity().getApplicationContext(), "Error Getting Data", Toast.LENGTH_SHORT);
            }
        } else {
            mTabHost.clearAllTabs();
            //Load Overview
            Bundle overviewArgs = new Bundle();
            mTabHost.addTab(mTabHost.newTabSpec("Tab1").setIndicator("All"),
                    AgendaFragment.class, overviewArgs);
        }
    }
}

