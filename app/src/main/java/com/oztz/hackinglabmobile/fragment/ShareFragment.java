package com.oztz.hackinglabmobile.fragment;

import android.app.Activity;
import android.app.ProgressDialog;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.database.Cursor;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.net.Uri;
import android.os.Bundle;
import android.os.Environment;
import android.provider.MediaStore;
import android.support.v4.app.Fragment;
import android.text.Editable;
import android.text.TextWatcher;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.EditText;
import android.widget.ImageButton;
import android.widget.ImageView;
import android.widget.TextView;
import android.widget.Toast;

import com.google.gson.Gson;
import com.oztz.hackinglabmobile.MainActivity;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.Media;
import com.oztz.hackinglabmobile.businessclasses.Social;
import com.oztz.hackinglabmobile.database.DbOperator;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.PostMediaTask;
import com.oztz.hackinglabmobile.helper.PostTask;

import java.io.File;
import java.io.IOException;
import java.text.SimpleDateFormat;
import java.util.Date;

/**
 * Created by Tobi on 20.03.2015.
 */
public class ShareFragment extends Fragment implements HttpResult {

    private static final String ARG_SECTION_NUMBER = "section_number";
    static final int REQUEST_IMAGE_CAPTURE = 1;
    static final int REQUEST_IMAGE_PICK = 2;

    private EditText postEditText;
    private ImageButton cameraButton, galleryButton;
    private Button shareButton;
    private TextView counter;
    private ImageView thumbnail;
    private String mediaUri;
    private String socialPost;
    private boolean imageUploaded = false;
    private String qrCode;
    ProgressDialog sending;

    public static ShareFragment newInstance(int sectionNumber) {
        Log.d("DEBUG", "PlaceholderFragment.newInstance(" + String.valueOf(sectionNumber) + ")");
        ShareFragment fragment = new ShareFragment();
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
        View view = inflater.inflate(R.layout.fragment_share, container, false);
        counter = (TextView) view.findViewById(R.id.share_text_counter);
        postEditText = (EditText) view.findViewById(R.id.social_post_editText);
        postEditText.addTextChangedListener(new TextWatcher() {
            @Override
            public void beforeTextChanged(CharSequence s, int start, int count, int after) {

            }

            @Override
            public void onTextChanged(CharSequence s, int start, int before, int count) {

            }

            @Override
            public void afterTextChanged(Editable s) {
                counter.setText(String.valueOf(s.length()) + "/140");
            }
        });
        thumbnail = (ImageView) view.findViewById(R.id.share_img_thumbnail);
        shareButton = (Button) view.findViewById(R.id.share_button_share);
        shareButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                share();
            }
        });
        cameraButton = (ImageButton) view.findViewById(R.id.share_button_camera);
        cameraButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                TakePictureIntent();
            }
        });
        galleryButton = (ImageButton) view.findViewById(R.id.share_button_album);
        galleryButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                SelectPictureIntent();
            }
        });
        return view;
    }

    @Override
    public void onAttach(Activity activity) {
        super.onAttach(activity);
        ((MainActivity) activity).onSectionAttached(getArguments().getInt(
                ARG_SECTION_NUMBER));
    }

    @Override
    public void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode == REQUEST_IMAGE_CAPTURE && resultCode == Activity.RESULT_OK) {
            thumbnail.setImageBitmap(getCompressedImage(mediaUri));
        }
        else if (requestCode == REQUEST_IMAGE_PICK && resultCode == Activity.RESULT_OK &&
                data != null)
        {
            try {
                Uri selectedImage = data.getData();
                String[] filePathColumn = {MediaStore.Images.Media.DATA};
                Cursor c = getActivity().getContentResolver().query(selectedImage, filePathColumn,
                        null, null, null);
                c.moveToFirst();
                int columnIndex = c.getColumnIndex(filePathColumn[0]);
                mediaUri = c.getString(columnIndex);
                c.close();
                thumbnail.setImageBitmap(BitmapFactory.decodeFile(mediaUri));
            } catch (Exception e){
                Toast.makeText(getActivity().getApplicationContext(), "Couldn't load image",
                        Toast.LENGTH_SHORT).show();
                Log.d("DEBUG", e.getMessage());
            }
        }
    }

    private File createImageFile() throws IOException {
        // Create an image file name
        String imageFileName = new SimpleDateFormat("yyyyMMdd_HHmmss").format(new Date());
        File storageDir = Environment.getExternalStoragePublicDirectory(
                Environment.DIRECTORY_PICTURES);
        File image = File.createTempFile(
                imageFileName,  /* prefix */
                ".jpg",         /* suffix */
                storageDir      /* directory */
        );

        // Save a file: path for use with ACTION_VIEW intents
        mediaUri = image.getAbsolutePath();
        return image;
    }

    private Bitmap getCompressedImage(String mediaPath){
        File file = new File(mediaPath);
        Bitmap source = BitmapFactory.decodeFile(mediaPath);

        double sourceWidth = source.getWidth();
        double sourceHeight = source.getHeight();
        if(sourceWidth > 1000 || sourceHeight > 1000){
            double max = Math.max(sourceWidth, sourceHeight);
            double scaleFactor = 1000 / max;
            int scaledWidth = (int)(sourceWidth * scaleFactor);
            int scaledHeight = (int)(sourceHeight * scaleFactor);
            source = Bitmap.createScaledBitmap(source, scaledWidth, scaledHeight, true);
        }
        return source;
    }


    private void share(){
        try {
            sending = ProgressDialog.show(getActivity(), "Sharing", "Sending your post...", true, true);
        } catch (Exception e){
            Log.d("DEBUG", e.getMessage());
        }
        DbOperator operator = new DbOperator(getActivity().getApplicationContext());
        qrCode = operator.getQrCode("author", App.eventId);
        socialPost = postEditText.getText().toString();
        if(mediaUri != null || (socialPost != null && socialPost.length() > 0)) { // Avoid empty posts
            if (mediaUri != null) {
                new PostMediaTask(this).execute(getResources().getString(R.string.rootURL) + "media/upload", mediaUri);
            } else {
                Social s = new Social(socialPost, "pending", null, App.username, App.userId, 0, App.eventId);
                if(qrCode != null) {
                    new PostTask(this).execute(getResources().getString(R.string.rootURL) + "social", new Gson().toJson(s), qrCode);
                } else {
                    new PostTask(this).execute(getResources().getString(R.string.rootURL) + "social", new Gson().toJson(s));
                }
            }
        }
    }

    private void TakePictureIntent() {
        PackageManager pm = getActivity().getPackageManager();
        if(!pm.hasSystemFeature(PackageManager.FEATURE_CAMERA)){
            Toast.makeText(getActivity().getApplicationContext(),
                    "No camera found!", Toast.LENGTH_SHORT).show();
            return;
        }
        Intent takePictureIntent = new Intent(MediaStore.ACTION_IMAGE_CAPTURE);
        if (takePictureIntent.resolveActivity(pm) != null) {
            File photoFile = null;
            try{
                photoFile = createImageFile();
            } catch (IOException e){
                Log.d("DEBUG", e.getMessage());
            }

            if(photoFile != null){
                takePictureIntent.putExtra(MediaStore.EXTRA_OUTPUT,
                        Uri.fromFile(photoFile));
            }
            startActivityForResult(takePictureIntent, REQUEST_IMAGE_CAPTURE);
        }
    }

    private void SelectPictureIntent() {
        PackageManager pm = getActivity().getPackageManager();
        Intent photoPickerIntent = new Intent(Intent.ACTION_PICK);
        photoPickerIntent.setType("image/*");
        startActivityForResult(photoPickerIntent, REQUEST_IMAGE_PICK);
    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        if(requestCode.equals("POST_MEDIA")){
            imageUploaded = true;
            try {
                Media m = new Gson().fromJson(JsonString, Media.class);
                Social s = new Social(socialPost, "pending", m.link, App.username, App.userId, m.mediaID, App.eventId);

                if(qrCode != null){
                    new PostTask(this).execute(getResources().getString(R.string.rootURL) + "social", new Gson().toJson(s), qrCode);
                } else {
                    new PostTask(this).execute(getResources().getString(R.string.rootURL) + "social", new Gson().toJson(s));
                }


            } catch (Exception e){
                Log.d("DEBUG", e.getMessage());
            }
        } else if(requestCode.equals("POST")){
            if(sending != null && sending.isShowing()) {
                sending.dismiss();
            }
            if(JsonString == null){
                Toast.makeText(App.getContext(), getResources().getString(R.string.error_share_failed), Toast.LENGTH_LONG).show();
            } else {
                Toast.makeText(App.getContext(), getResources().getString(R.string.share_success), Toast.LENGTH_LONG).show();
                getFragmentManager().beginTransaction()
                        .replace(R.id.container, MainFragment.newInstance(1))
                        .commit();
            }

        }
    }
}
