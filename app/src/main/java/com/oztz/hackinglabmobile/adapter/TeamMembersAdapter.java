package com.oztz.hackinglabmobile.adapter;

import android.content.Context;
import android.net.Uri;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ArrayAdapter;
import android.widget.ImageView;
import android.widget.TextView;

import com.nostra13.universalimageloader.core.DisplayImageOptions;
import com.nostra13.universalimageloader.core.ImageLoader;
import com.nostra13.universalimageloader.core.ImageLoaderConfiguration;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.Participant;

/**
 * Created by Tobi on 25.03.2015.
 */
public class TeamMembersAdapter extends ArrayAdapter {

    ImageLoader imageLoader;
    private final Context context;

    public TeamMembersAdapter(Context context, int resource, Participant[] participants) {
        super(context, resource, participants);
        this.context = context;
        imageLoader = ImageLoader.getInstance();
        ImageLoaderConfiguration config = new ImageLoaderConfiguration.Builder(context)
                .diskCacheFileCount(50)
                .defaultDisplayImageOptions(new DisplayImageOptions.Builder()
                        .cacheInMemory(true)
                        .cacheOnDisk(true).build())
                .build();
        imageLoader.init(config);
    }

    private static class ViewHolder {
        TextView name, nick, gender, age, skills;
        ImageView flag, memberImage;
        int id;
    }

    @Override
    public View getView(int position, View convertView, ViewGroup parent) {
        ViewHolder holder = new ViewHolder();
        LayoutInflater inflater = LayoutInflater.from(getContext());
        View v = inflater.inflate(R.layout.item_team_member, null);
        final Participant item = (Participant)getItem(position);

        if (item != null) {
            holder.name = (TextView) v.findViewById(R.id.team_member_name);
            holder.nick = (TextView) v.findViewById(R.id.team_member_nickname);
            holder.gender = (TextView) v.findViewById(R.id.team_member_gender);
            holder.age = (TextView) v.findViewById(R.id.team_member_age);
            holder.skills = (TextView) v.findViewById(R.id.team_member_skills);
            holder.flag = (ImageView) v.findViewById(R.id.team_member_flag);
            holder.memberImage = (ImageView) v.findViewById(R.id.team_member_portrait);

            if(holder.memberImage != null){
                imageLoader.displayImage(context.getResources().getString(R.string.teamMemberImageUrl) +
                    String.valueOf(item.userID) + ".jpg", holder.memberImage);
            }

            if (holder.name != null) {
                holder.name.setText(item.firstname + " " + item.lastname);
            }
            if(holder.nick != null){
                holder.nick.setText(item.nick);
            }
            if(holder.gender != null){
                holder.gender.setText(item.gender);
            }
            if(holder.age != null){
                holder.age.setText(String.valueOf(item.ageAsYear));
            }
            if(holder.skills != null){
                holder.skills.setText(String.valueOf(item.strength));
            }
            if (holder.flag != null) {
                holder.flag.setImageURI(Uri.parse("android.resource://com.oztz.hackinglabmobile/drawable/flag_"
                        + item.nationality.toLowerCase()));
            }
        }
        v.setTag(holder);
        return v;
    }
}
