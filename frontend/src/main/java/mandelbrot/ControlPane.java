package mandelbrot;

import javafx.geometry.Insets;

import javafx.scene.text.Font;

import javafx.scene.layout.GridPane;
import javafx.scene.control.TextField;
import javafx.scene.control.Label;
import javafx.scene.control.Button;
import javafx.scene.control.ComboBox;
import javafx.scene.control.ProgressBar;
import javafx.scene.layout.Region;
import javafx.scene.layout.Priority;

public class ControlPane extends GridPane {

    private TextField mIterationsTextField,
                      mImageWidthTextField,
                      mImageHeightTextField,
                      mSupersamplingTextField,
                      mCenterXTextField,
                      mCenterYTextField,
                      mRadiusTextField,
                      mColorShiftTextField,
                      mColorScaleTextField;

    private Label mIterationsLabel,
                  mImageWidthLabel,
                  mImageHeightLabel,
                  mSupersamplingLabel,
                  mCenterXLabel,
                  mCenterYLabel,
                  mRadiusLabel,
                  mColorShiftLabel,
                  mColorScaleLabel,
                  mColorFunctionLabel;

    private Button mRenderButton;
    private ComboBox mColorFunctionComboBox;
    private ProgressBar mRenderProgressBar;

    public ControlPane() {
        // Set the gap between each cell
        setVgap(3);
        setHgap(3);

        // Set the padding aroud the entire pane
        setPadding(new Insets(12));

        // Create the labels
        mIterationsLabel = makeLabel("Iterations");
        mImageWidthLabel = makeLabel("Image Width");
        mImageHeightLabel = makeLabel("Image Height");
        mSupersamplingLabel = makeLabel("Supersampling");
        mCenterXLabel = makeLabel("Center X");
        mCenterYLabel = makeLabel("Center Y");
        mRadiusLabel = makeLabel("Radius");
        mColorShiftLabel = makeLabel("Color Shift");
        mColorScaleLabel = makeLabel("Color Scale");
        mColorFunctionLabel = makeLabel("Color Function");

        // Create the text fields
        mIterationsTextField = makeTextField();
        mImageWidthTextField = makeTextField();
        mImageHeightTextField = makeTextField();
        mSupersamplingTextField = makeTextField();
        mCenterXTextField = makeTextField();
        mCenterYTextField = makeTextField();
        mRadiusTextField = makeTextField();
        mColorShiftTextField = makeTextField();
        mColorScaleTextField = makeTextField();
        
        // Create the "Render" button
        mRenderButton = new Button("Render");
        setFont(mRenderButton);

        // Create the Color Function combo box
        mColorFunctionComboBox = new ComboBox();

        // Create the progress bar
        mRenderProgressBar = new ProgressBar();

        // Add all the components to the GridView
        add(mIterationsLabel, 0, 0);
        add(mImageWidthLabel, 0, 1);
        add(mImageHeightLabel, 0, 2);
        add(mSupersamplingLabel, 0, 3);
        add(mCenterXLabel, 3, 0);
        add(mCenterYLabel, 3, 1);
        add(mRadiusLabel, 3, 2);
        add(mColorFunctionLabel, 6, 0);
        add(mColorShiftLabel, 6, 1);
        add(mColorScaleLabel, 6, 2);

        add(mIterationsTextField, 1, 0);
        add(mImageWidthTextField, 1, 1);
        add(mImageHeightTextField, 1, 2);
        add(mSupersamplingTextField, 1, 3);
        add(mCenterXTextField, 4, 0);
        add(mCenterYTextField, 4, 1);
        add(mRadiusTextField, 4, 2);
        add(mColorFunctionComboBox, 7, 0);
        add(mColorShiftTextField, 7, 1);
        add(mColorScaleTextField, 7, 2);

        add(mRenderButton, 7, 3);
        add(mRenderProgressBar, 3, 3, 4, 1);

        mRenderProgressBar.setPrefWidth(200);
        mRenderProgressBar.setPrefHeight(25);

        // Add spacers between groups of columns
        Region sep1 = new Region();
        sep1.setPrefWidth(36);
        add(sep1, 2, 0);
        Region sep2 = new Region();
        sep2.setPrefWidth(36);
        add(sep2, 5, 0);
    }

    private static Label makeLabel(String text) {
        Label label = new Label(text);
        setFont(label);
        return label;
    }

    private static TextField makeTextField() {
        TextField textField = new TextField();
        return textField;
    }

    private static void setFont(javafx.scene.control.Labeled component) {
        component.setFont(Font.loadFont(App.class.getClassLoader().getResource("fonts/NotoSans-Regular.ttf").toExternalForm(), 14));
    }
    
}
