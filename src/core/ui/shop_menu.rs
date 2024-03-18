use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, epaint::RectShape, util::undoer::Settings, Align2, Button, Color32, Grid, Image,
        Margin, Painter, Pos2, Rect, RichText, Rounding, Stroke, Style, TextureOptions, Vec2,
    },
    EguiContexts,
};

use crate::core::{
    items::{Inventory, Item, ItemPrices},
    GameStage,
};

pub fn shop_menu_system(
    mut contexts: EguiContexts,
    mut next_game_stage: ResMut<NextState<GameStage>>,
    mut inventory: ResMut<Inventory>,
    item_prices: Res<ItemPrices>,
) {
    egui::Area::new("shop_menu")
        .anchor(Align2::CENTER_CENTER, [32., 0.])
        .order(egui::Order::Background)
        .show(contexts.ctx_mut(), |ui| {
            // ui.set_height(200.);
            ui.set_width(160. * 4. + 16.);
            egui::Frame::none()
                .inner_margin(Margin::same(8.))
                .outer_margin(Margin::same(0.))
                .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 200))
                .show(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        // ui.add(yarn_icon.clone());
                        ui.label(
                            RichText::new("Yarn: ".to_owned() + &inventory.yarn.to_string())
                                .size(32.),
                        );
                    });
                    ui.add_space(16.);
                    Grid::new("shop_menu_grid")
                        .min_col_width(160.)
                        .show(ui, |ui| {
                            // ui.heading(RichText::new("Shop").size(36.));
                            // ui.end_row();

                            let yarn_icon = Image::new(egui::include_image!(
                                "../../../assets/sprites/yarn/yarn.png"
                            ));
                            ui.add_enabled_ui(
                                !inventory.items.contains(&Item::SodaBooster),
                                |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label(RichText::new("Buy Soda Boost").size(18.));
                                        let res = ui
                                            .add(Button::image_and_text(
                                                yarn_icon.clone(),
                                                item_prices
                                                    .0
                                                    .get(&Item::SodaBooster)
                                                    .unwrap_or(&-1)
                                                    .to_string(),
                                            ))
                                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        if res.clicked() {
                                            let _ = inventory
                                                .buy_item(Item::SodaBooster, &item_prices.0);
                                        }
                                    });
                                },
                            );
                            ui.add_enabled_ui(
                                inventory.items.contains(&Item::SodaBooster)
                                    && !inventory.items.contains(&Item::FireworkBooster),
                                |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label(RichText::new("Buy Fireworks").size(18.));
                                        let res = ui
                                            .add(Button::image_and_text(
                                                yarn_icon.clone(),
                                                item_prices
                                                    .0
                                                    .get(&Item::FireworkBooster)
                                                    .unwrap_or(&-1)
                                                    .to_string(),
                                            ))
                                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        if res.clicked() {
                                            let _ = inventory
                                                .buy_item(Item::FireworkBooster, &item_prices.0);
                                        }
                                    });
                                },
                            );
                            ui.add_enabled_ui(
                                inventory.items.contains(&Item::FireworkBooster)
                                    && !inventory.items.contains(&Item::RocketBooster),
                                |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label(RichText::new("Buy Rocket").size(18.));
                                        let res = ui
                                            .add(Button::image_and_text(
                                                yarn_icon.clone(),
                                                item_prices
                                                    .0
                                                    .get(&Item::RocketBooster)
                                                    .unwrap_or(&-1)
                                                    .to_string(),
                                            ))
                                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        if res.clicked() {
                                            let _ = inventory
                                                .buy_item(Item::RocketBooster, &item_prices.0);
                                        }
                                    });
                                },
                            );
                            ui.end_row();

                            ui.add_enabled_ui(
                                !inventory.items.contains(&Item::RacingVehicle),
                                |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label(RichText::new("Buy Racer Box").size(18.));
                                        let res = ui
                                            .add(Button::image_and_text(
                                                yarn_icon.clone(),
                                                item_prices
                                                    .0
                                                    .get(&Item::RacingVehicle)
                                                    .unwrap_or(&-1)
                                                    .to_string(),
                                            ))
                                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        if res.clicked() {
                                            let _ = inventory
                                                .buy_item(Item::RacingVehicle, &item_prices.0);
                                        }
                                    });
                                },
                            );
                            ui.add_enabled_ui(
                                inventory.items.contains(&Item::RacingVehicle)
                                    && !inventory.items.contains(&Item::SkiingVehicle),
                                |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label(RichText::new("Buy Skiing Box").size(18.));
                                        let res = ui
                                            .add(Button::image_and_text(
                                                yarn_icon.clone(),
                                                item_prices
                                                    .0
                                                    .get(&Item::SkiingVehicle)
                                                    .unwrap_or(&-1)
                                                    .to_string(),
                                            ))
                                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        if res.clicked() {
                                            let _ = inventory
                                                .buy_item(Item::SkiingVehicle, &item_prices.0);
                                        }
                                    });
                                },
                            );
                            ui.end_row();

                            ui.add_enabled_ui(
                                !inventory.items.contains(&Item::SingleBalloon),
                                |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label(RichText::new("Buy Balloon").size(18.));
                                        let res = ui
                                            .add(Button::image_and_text(
                                                yarn_icon.clone(),
                                                item_prices
                                                    .0
                                                    .get(&Item::SingleBalloon)
                                                    .unwrap_or(&-1)
                                                    .to_string(),
                                            ))
                                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        if res.clicked() {
                                            let _ = inventory
                                                .buy_item(Item::SingleBalloon, &item_prices.0);
                                        }
                                    });
                                },
                            );
                            ui.add_enabled_ui(
                                inventory.items.contains(&Item::SingleBalloon)
                                    && !inventory.items.contains(&Item::TripleBalloons),
                                |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label(RichText::new("Buy 3x Balloon").size(18.));
                                        let res = ui
                                            .add(Button::image_and_text(
                                                yarn_icon.clone(),
                                                item_prices
                                                    .0
                                                    .get(&Item::TripleBalloons)
                                                    .unwrap_or(&-1)
                                                    .to_string(),
                                            ))
                                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        if res.clicked() {
                                            let _ = inventory
                                                .buy_item(Item::TripleBalloons, &item_prices.0);
                                        }
                                    });
                                },
                            );
                            ui.add_enabled_ui(
                                inventory.items.contains(&Item::TripleBalloons)
                                    && !inventory.items.contains(&Item::HotAirBalloon),
                                |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label(RichText::new("Buy Balloon Ship").size(18.));
                                        let res = ui
                                            .add(Button::image_and_text(
                                                yarn_icon.clone(),
                                                item_prices
                                                    .0
                                                    .get(&Item::HotAirBalloon)
                                                    .unwrap_or(&-1)
                                                    .to_string(),
                                            ))
                                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        if res.clicked() {
                                            let _ = inventory
                                                .buy_item(Item::HotAirBalloon, &item_prices.0);
                                        }
                                    });
                                },
                            );

                            ui.add_enabled_ui(
                                inventory.items.contains(&Item::HotAirBalloon)
                                    && !inventory.items.contains(&Item::GliderBalloon),
                                |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label(RichText::new("Buy Glider").size(18.));
                                        let res = ui
                                            .add(Button::image_and_text(
                                                yarn_icon.clone(),
                                                item_prices
                                                    .0
                                                    .get(&Item::GliderBalloon)
                                                    .unwrap_or(&-1)
                                                    .to_string(),
                                            ))
                                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        if res.clicked() {
                                            let _ = inventory
                                                .buy_item(Item::GliderBalloon, &item_prices.0);
                                        }
                                    });
                                },
                            );
                        });
                    ui.add_space(16.);
                    ui.vertical_centered(|ui| {
                        if ui
                            .button(RichText::new("Finish").size(16.))
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            next_game_stage.set(GameStage::Sledding);
                        }
                    });
                });
        });
}
