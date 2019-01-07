package mandelbrot;

import java.util.ArrayList;
import java.util.Collections;
import java.io.File;

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
import javafx.scene.image.Image;
import javafx.scene.canvas.GraphicsContext;

public class ControlPane extends GridPane {

    private boolean mEnabled = true;

    private TextField mIterationsTextField,
                      mImageWidthTextField,
                      mImageHeightTextField,
                      mSupersamplingTextField,
                      mCenterXTextField,
                      mCenterYTextField,
                      mRadiusTextField,
                      mColorShiftTextField,
                      mColorScaleTextField;

    private String mSavedIterations,
                   mSavedImageWidth,
                   mSavedImageHeight,
                   mSavedSupersampling,
                   mSavedCenterX,
                   mSavedCenterY,
                   mSavedRadius,
                   mSavedColorShift,
                   mSavedColorScale,
                   mSavedColorFunction;

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

    private ArrayList<TextField> mPossiblyInvalidTextFields = new ArrayList<>();

    private SocketComm mSocketComm;

    private enum Field {
        Iterations, Width, Height, Supersampling, CenterX, CenterY, 
        Radius, ColorShift, ColorScale, ColorFunction
    }

    public ControlPane(SocketComm socketComm, GraphicsContext imageGC, App app) {
        mSocketComm = socketComm;

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
        mIterationsTextField = makeTextField(Field.Iterations, 500, 0, Integer.MAX_VALUE);
        mImageWidthTextField = makeTextField(Field.Width, 1000, 0, Integer.MAX_VALUE);
        mImageHeightTextField = makeTextField(Field.Height, 1000, 0, Integer.MAX_VALUE);
        mSupersamplingTextField = makeTextField(Field.Supersampling, 1, 1, 8);
        mCenterXTextField = makeTextField(Field.CenterX, 0.0, Double.NEGATIVE_INFINITY, Double.POSITIVE_INFINITY);
        mCenterYTextField = makeTextField(Field.CenterY, 0.0, Double.NEGATIVE_INFINITY, Double.POSITIVE_INFINITY);
        mRadiusTextField = makeTextField(Field.Radius, 2.0, 0.0, Double.POSITIVE_INFINITY);
        mColorShiftTextField = makeTextField(Field.ColorShift, 0, 0, 2047);
        mColorScaleTextField = makeTextField(Field.ColorScale, 32.0, Double.NEGATIVE_INFINITY, Double.POSITIVE_INFINITY);
        
        // Create the "Render" button
        mRenderButton = new Button("Render");
        setFont(mRenderButton);

        // Create the Color Function combo box
        mColorFunctionComboBox = new ComboBox();

        // Create the progress bar
        mRenderProgressBar = new ProgressBar(0.0);

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

        // Add options to selection dropdown for color function
        mColorFunctionComboBox.getItems().addAll(
            "Greyscale", "Reversed greyscale", "Colorized", "Red");
        mColorFunctionComboBox.setValue("Colorized");

        mColorFunctionComboBox.valueProperty().addListener((observed, oldValue, newValue) -> {
            ControlPane.this.updateColorInputs((String) newValue);
            updateButton();
        });

        // Set Button action
        mRenderButton.setOnAction((event) -> {
            new Thread() {
                public void run() {
                    // Set zero progress
                    mRenderProgressBar.setProgress(0F);

                    // Disable the UI
                    ControlPane.this.setEnabled(false);
                    saveState();

                    // Send the render message
                    final boolean result = renderRequest();

                    // Track the progress
                    while (result) {
                        double progress = progressRequest();

                        // Check for error or done
                        if (progress < 0.0) {
                            // No operation pending
                            mRenderProgressBar.setProgress(0F);
                            break;
                        } else if (progress > 100.0) {
                            // Done
                            mRenderProgressBar.setProgress(1F);
                            break;
                        } else {
                            // Update progressbar
                            mRenderProgressBar.setProgress(progress / 100.0);
                        }
                    }

                    // Request the rendered data file path
                    String output = outputRequest();

                    System.out.println("---" + output + "---");

                    // Show the image
                    if (!output.isEmpty()) {
                        // Show image
                        File file = new File(output);

                        try {
                            Image img = new Image(file.toURI().toURL().toExternalForm(), 800, 800, true, true);
                            imageGC.drawImage(img, 0, 0);
                            app.resetDrawCanvas();
                        } catch (Exception e) {
                            e.printStackTrace();
                        }
                    }

                    // Enable the UI
                    ControlPane.this.setEnabled(true);
                }
            }.start();
        });

        saveState();
    }

    private static Label makeLabel(String text) {
        Label label = new Label(text);
        setFont(label);
        return label;
    }

    private TextField makeTextField(Field field, int defaultValue, int min, int max) {
        TextField textField = new TextField();
        // Add change listener
        textField.textProperty().addListener((observed, oldText, newText) -> {
            // Remove non-numeric characters
            if (!newText.matches("-?\\d*")) {
                newText = newText.replaceAll("[^-\\d]", "");
            }

            textField.setText(newText);
        });
        
        // Add focus listener
        textField.focusedProperty().addListener((observed, oldValue, newValue) -> {
            if (!newValue) {
                // Make sure it contains a valid number
                try {
                    String text = textField.getText();
                    int parsed = Integer.parseInt(text);

                    if (parsed < min || parsed > max) {
                        setInvalid(textField);
                    } else {
                        setValid(textField);
                    }
                } catch (Exception e) {
                    setInvalid(textField);
                }
            }
        });

        textField.setText("" + defaultValue);

        return textField;
    }

    private TextField makeTextField(Field field, double defaultValue, double min, double max) {
        TextField textField = new TextField();
        // Add change listener
        textField.textProperty().addListener((observed, oldText, newText) -> {
            // Don't let invalid characters be entered
            if (!newText.matches("-?(\\d*)?(\\.(\\d*))?")) {
                newText = oldText;
            }

            textField.setText(newText);
        });

        // Add focus listener
        textField.focusedProperty().addListener((observed, oldValue, newValue) -> {
            if (!newValue) {
                // Make sure it contains a valid number
                try {
                    String text = textField.getText();
                    double parsed = Double.parseDouble(text);

                    if (parsed < min || parsed > max) {
                        setInvalid(textField);
                    } else {
                        setValid(textField);
                    }
                } catch (Exception e) {
                    setInvalid(textField);
                }
            }
        });

        textField.setText("" + defaultValue);

        return textField;
    }

    private void setInvalid(TextField textField) {
        textField.setStyle("-fx-text-box-border: red; -fx-focus-color: red;");
        if (!textField.getStyleClass().contains("invalid")) {
            textField.getStyleClass().add("invalid");
        }
        if (!this.mPossiblyInvalidTextFields.contains(textField)) {
            this.mPossiblyInvalidTextFields.add(textField);
        }
        updateButton();
    }

    private void setValid(TextField textField) {
        textField.setStyle("");
        textField.getStyleClass().removeAll(Collections.singleton("invalid"));
        updateButton();
    }

    private void updateButton() {
        boolean flag = true;

        for (TextField textField : this.mPossiblyInvalidTextFields) {
            if (textField.getStyleClass().contains("invalid")) {
                flag = false;
                break;
            }
        }

        this.mRenderButton.setDisable(!flag);
    }

    public void setEnabled(boolean enabled) {
        if (!enabled) {
            // Disable everything
            mIterationsTextField.setDisable(true);
            mImageWidthTextField.setDisable(true);
            mImageHeightTextField.setDisable(true);
            mSupersamplingTextField.setDisable(true);
            mCenterXTextField.setDisable(true);
            mCenterYTextField.setDisable(true);
            mRadiusTextField.setDisable(true);
            mColorShiftTextField.setDisable(true);
            mColorScaleTextField.setDisable(true);
            mColorFunctionComboBox.setDisable(true);
            mRenderButton.setDisable(true);
        } else {
            // Enable everything except color text fields and render button
            mIterationsTextField.setDisable(false);
            mImageWidthTextField.setDisable(false);
            mImageHeightTextField.setDisable(false);
            mSupersamplingTextField.setDisable(false);
            mCenterXTextField.setDisable(false);
            mCenterYTextField.setDisable(false);
            mRadiusTextField.setDisable(false);
            mColorFunctionComboBox.setDisable(false);

            String colorFunc = (String) mColorFunctionComboBox.getValue();
            // Check colorfunction to see if we should enable color text fields
            updateColorInputs(colorFunc);
            updateButton();
        }

        mEnabled = enabled;
    }

    public boolean isEnabled() {
        return mEnabled;
    }

    private void saveState() {
        mSavedIterations = mIterationsTextField.getText();
        mSavedImageWidth = mImageWidthTextField.getText();
        mSavedImageHeight = mImageHeightTextField.getText();
        mSavedSupersampling = mSupersamplingTextField.getText();
        mSavedCenterX = mCenterXTextField.getText();
        mSavedCenterY = mCenterYTextField.getText();
        mSavedRadius = mRadiusTextField.getText();
        mSavedColorShift = mColorShiftTextField.getText();
        mSavedColorScale = mColorScaleTextField.getText();
        mSavedColorFunction = (String) mColorFunctionComboBox.getValue();
    }

    private void updateColorInputs(String colorFunc) {
        if (colorFunc.equals("Colorized") || colorFunc.equals("Red")) {
            // Enable text fields
            mColorShiftTextField.setDisable(false);
            mColorScaleTextField.setDisable(false);

            // Change "invalid_disabled" class to "invalid"
            if (mColorShiftTextField.getStyleClass().contains("invalid_disabled")) {
                mColorShiftTextField.getStyleClass().removeAll(Collections.singleton("invalid_disabled"));
                mColorShiftTextField.getStyleClass().add("invalid");
            }
            if (mColorScaleTextField.getStyleClass().contains("invalid_disabled")) {
                mColorScaleTextField.getStyleClass().removeAll(Collections.singleton("invalid_disabled"));
                mColorScaleTextField.getStyleClass().add("invalid");
            }
        } else {
            // Disable text fields
            mColorShiftTextField.setDisable(true);
            mColorScaleTextField.setDisable(true);

            // Change "invalid" class to "invalid_disabled"
            if (mColorShiftTextField.getStyleClass().contains("invalid")) {
                mColorShiftTextField.getStyleClass().removeAll(Collections.singleton("invalid"));
                mColorShiftTextField.getStyleClass().add("invalid_disabled");
            }
            if (mColorScaleTextField.getStyleClass().contains("invalid")) {
                mColorScaleTextField.getStyleClass().removeAll(Collections.singleton("invalid"));
                mColorScaleTextField.getStyleClass().add("invalid_disabled");
            }
        }
    }

    private boolean renderRequest() {
        String request = "render ";

        request += mIterationsTextField.getText() + " "
            + mImageWidthTextField.getText() + " "
            + mImageHeightTextField.getText() + " "
            + mSupersamplingTextField.getText() + " "
            + mCenterXTextField.getText() + " "
            + mCenterYTextField.getText() + " "
            + mRadiusTextField.getText() + " ";

        request += formatColorFunction();

        String response = "";

        try {
            response = mSocketComm.sendAndReceive(request);
        } catch (Exception e) {
            e.printStackTrace();
            return false;
        }

        if (response.equals("ok")) {
            return true;
        } else {
            System.err.println(response);
            // TODO: show the error message
            return false;
        }
    }

    private double progressRequest() {
        String request = "progress";
        String response = "";

        try {
            response = mSocketComm.sendAndReceive(request);

            if (response.trim().equals("error(4)")) {
                // There's no operation happening right now
                return -1.0;
            }

            return Double.parseDouble(response);
        } catch (Exception e) {
            e.printStackTrace();
            return -1.0;
        }
    }

    private String outputRequest() {
        String request = "output";
        String response = "";

        try {
            while (true) {
                response = mSocketComm.sendAndReceive(request);
            
                if (response.trim().equals("error(5)")) {
                    return "";
                } else if (response.trim().equals("error(6.2)")) {
                    continue;
                }

                break;
            }

            return response;
        } catch (Exception e) {
            e.printStackTrace();
            return "";
        }
    }

    private String formatColorFunction() {
        String selected = (String) mColorFunctionComboBox.getValue();

        if (selected.equals("Greyscale")) {
            return "greyscale";
        } else if (selected.equals("Reversed Greyscale")) {
            return "rgreyscale";
        } else if (selected.equals("Colorized")) {
            String shift = mColorShiftTextField.getText();
            String scale = mColorScaleTextField.getText();
            return "color(" + shift + "," + scale + ")";
        } else if (selected.equals("Red")) {
            String shift = mColorShiftTextField.getText();
            String scale = mColorScaleTextField.getText();
            return "red(" + shift + "," + scale + ")";
        } else {
            // What??
            mColorFunctionComboBox.setValue("Greyscale");
            return formatColorFunction();
        }
    }

    private static void setFont(javafx.scene.control.Labeled component) {
        component.setFont(Font.loadFont(App.class.getClassLoader().getResource("fonts/NotoSans-Regular.ttf").toExternalForm(), 14));
    }

    public double getSavedAspect() {
        try {
            return Double.parseDouble(mSavedImageWidth) / Double.parseDouble(mSavedImageHeight);
        } catch (Exception e) {
            return 1.0;
        }
    }
    
}
