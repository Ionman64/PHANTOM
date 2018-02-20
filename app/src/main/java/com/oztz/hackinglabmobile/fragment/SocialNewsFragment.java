package com.oztz.hackinglabmobile.fragment;

import android.os.Bundle;
import android.support.v4.app.Fragment;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ImageButton;
import android.widget.ListView;
import android.widget.Toast;

import com.google.gson.Gson;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.adapter.SocialAdapter;
import com.oztz.hackinglabmobile.businessclasses.Social;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.RequestTask;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;

/**
 * Created by Tobi on 20.03.2015.
 */
public class SocialNewsFragment extends Fragment implements HttpResult {

    ListView SocialNewsListView;
    ImageButton shareButton;
    View footer;

    @Override
    public void onCreate(Bundle savedInstanceState) {
        Log.d("DEBUG", "SocialNewsFragment.onCreate()");
        super.onCreate(savedInstanceState);
    }

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container,
                             Bundle savedInstanceState)
    {
        Log.d("DEBUG", "SocialNewsFragment.onCreateView()");
        View view = inflater.inflate(R.layout.fragment_socialnews, container, false);
        SocialNewsListView = (ListView) view.findViewById(R.id.SocialNews_List_View);
        shareButton = (ImageButton) view.findViewById(R.id.socialNews_imageButton_share);
        shareButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                loadShareFragment();
            }
        });
        String url = getResources().getString(R.string.rootURL) + "event/" +
                String.valueOf(App.eventId) + "/socials/newest";
        updateView(App.db.getContentFromDataBase(url), "db");
        new RequestTask(this).execute(url, "newest");

        return view;
    }

    private void loadMoreData(){
        new RequestTask(this).execute(getResources().getString(R.string.rootURL) + "event/" +
                String.valueOf(App.eventId) + "/socials", "all");
    }

    private void loadShareFragment(){
        getActivity().getSupportFragmentManager().beginTransaction()
                .replace(R.id.container,
                        ShareFragment.newInstance(3))
                .commit();
    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        if(JsonString != null) {
            updateView(JsonString, requestCode);
        } else {
            Toast.makeText(getActivity().getApplicationContext(), "Error Getting Data",Toast.LENGTH_SHORT);
        }
    }

    private Social[] getPublishedNews(Social[] socials){
        List<Social> socialList = new ArrayList<Social>();
        for(int i=0; i<socials.length; i++){
            if(socials[i].status.equals("accepted") || socials[i].status.equals("published")){
                socialList.add(socials[i]);
            }
        }
        return socialList.toArray(new Social[socialList.size()]);
    }

    private void updateView(String json, String requestCode){
        if(json != null){
            try {
                Social[] socialnews = new Gson().fromJson(json, Social[].class);
                socialnews = getPublishedNews(socialnews);
                Arrays.sort(socialnews, new Comparator<Social>() {
                    @Override
                    public int compare(Social lhs, Social rhs) {
                        return lhs.socialID - rhs.socialID;
                    }
                });

                if(requestCode.equals("newest")){
                    if(SocialNewsListView.getFooterViewsCount() == 0 && socialnews.length >= App.newestSelectLimit){
                        LayoutInflater inflater = LayoutInflater.from(getActivity().getApplicationContext());
                        footer = inflater.inflate(R.layout.item_load_more_data, null);
                        footer.setOnClickListener(new View.OnClickListener() {
                            @Override
                            public void onClick(View v) {
                                loadMoreData();
                            }
                        });
                        SocialNewsListView.addFooterView(footer);
                    }
                    SocialNewsListView.setAdapter(new SocialAdapter(getActivity(), R.layout.item_article_textonly, socialnews));
                } else if(requestCode.equals("all")){
                    int scrollPosition = SocialNewsListView.getFirstVisiblePosition();
                    SocialNewsListView.removeFooterView(footer);
                    SocialNewsListView.setAdapter(new SocialAdapter(getActivity(), R.layout.item_article_textonly, socialnews));
                    SocialNewsListView.setSelectionFromTop(scrollPosition+1, 0);
                } else if(requestCode.equals("db")){
                    SocialNewsListView.setAdapter(new SocialAdapter(getActivity(), R.layout.item_article_textonly, socialnews));
                }
            } catch (Exception e){
                Toast.makeText(getActivity(), "Error loading data", Toast.LENGTH_SHORT);
            }
        }
    }
}
