use bevy::{
    ecs::system::{EntityCommand, EntityCommands},
    prelude::*,
};
use sickle_ui::{
    ui_builder::{UiBuilder, UiRoot},
    ui_commands::SetTextExt,
    ui_style::{
        SetImageExt, SetNodeAlignSelfExt, SetNodeHeightExt, SetNodeJustifyContentsExt,
        SetNodePositionTypeExt, SetNodeTopExt, SetNodeWidthExt,
    },
    widgets::{
        container::UiContainerExt,
        label::{LabelConfig, UiLabelExt},
    },
};

pub(super) fn plugin(app: &mut App) {
    app //
        .add_systems(Update, init_label);
}

#[derive(Component)]
pub struct BannerWidget;

// A marker component used internally to initialize the label font.
#[derive(Component)]
struct BannerLabel;

pub struct BannerWidgetConfig {
    pub label: String,
    // Other options can be added here...
}

impl BannerWidgetConfig {
    pub fn from(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

pub trait UiBannerWidgetExt<'w, 's> {
    fn banner_widget<'a>(&'a mut self, config: BannerWidgetConfig)
        -> UiBuilder<'w, 's, 'a, Entity>;
}

impl<'w, 's> UiBannerWidgetExt<'w, 's> for UiBuilder<'w, 's, '_, UiRoot> {
    fn banner_widget<'a>(
        &'a mut self,
        config: BannerWidgetConfig,
    ) -> UiBuilder<'w, 's, 'a, Entity> {
        self.container((ImageBundle::default(), BannerWidget), |banner| {
            banner
                .style()
                .position_type(PositionType::Absolute)
                // Center the children (the label) horizontally.
                .justify_content(JustifyContent::Center)
                .width(Val::Px(401.0))
                .height(Val::Px(79.0))
                // Add a nice looking background image to our widget.
                .image("banner_title.png");

            // And we'll want a customizable label on the banner.
            let mut label = banner.label(LabelConfig::default());

            label
                .style()
                // Align the label relative to the top of the banner.
                .align_self(AlignSelf::Start)
                // Move us a few pixels down so we look nice relative to our font.
                .top(Val::Px(10.0));

            // We would like to set a default text style without having to pass in the AssetServer.
            label
                .entity_commands()
                .insert(BannerLabel)
                .set_text(config.label, None);
        })
    }
}

fn init_label(
    mut banner_labels: Query<&mut Text, Added<BannerLabel>>,
    asset_server: Res<AssetServer>,
) {
    // Delayed initialization of the label font.
    // This allows us to set the font style without having to pass in the AssetServer to our widget builder method.
    for mut text in &mut banner_labels {
        for text_section in &mut text.sections {
            text_section.style = TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::rgb(0.471, 0.278, 0.153),
            }
        }
    }
}

// This extension trait lets us call set_position on an entity command queue for a banner widget.
// (Really, this is not constrained to just a banner widget and could be used on any widget.)
pub trait BannerWidgetCommands<'a> {
    fn set_position(&'a mut self, x: f32, y: f32) -> &mut EntityCommands<'a>;
}

impl<'a> BannerWidgetCommands<'a> for EntityCommands<'a> {
    fn set_position(&'a mut self, x: f32, y: f32) -> &mut EntityCommands<'a> {
        // We insert our custom command into the entity commands queue.
        self.add(SetPosition(x, y))
    }
}

struct SetPosition(f32, f32);

impl EntityCommand for SetPosition {
    fn apply(self, entity: Entity, world: &mut World) {
        // Commands work with direct access to the world.
        // We can set the position by modifying the style directly:
        if let Some(mut style) = world.entity_mut(entity).get_mut::<Style>() {
            style.position_type = PositionType::Absolute;
            style.left = Val::Px(self.0);
            style.top = Val::Px(self.1);
            style.right = Val::Auto;
            style.bottom = Val::Auto;
        }

        // Because you have access to the world, you could get resources here, load assets,
        // do anything you need to modify your widget in an interesting way.
    }
}
