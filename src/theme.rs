use egui::{Color32, Pos2, Stroke, Vec2};
pub fn apply(ctx: &egui::Context, dark: bool) {
    let (canvas, surface, muted, ink, accent) = if dark {
        (Color32::from_rgb(20,27,24), Color32::from_rgb(27,36,32), Color32::from_rgb(35,47,42), Color32::from_rgb(241,245,242), Color32::from_rgb(239,154,115))
    } else {
        (Color32::from_rgb(244,241,233), Color32::from_rgb(255,254,250), Color32::from_rgb(235,236,229), Color32::from_rgb(23,35,31), Color32::from_rgb(184,74,36))
    };
    let mut v = if dark { egui::Visuals::dark() } else { egui::Visuals::light() };
    v.override_text_color=Some(ink); v.panel_fill=canvas; v.window_fill=surface; v.extreme_bg_color=surface;
    v.widgets.noninteractive.bg_fill=surface; v.widgets.inactive.bg_fill=muted; v.selection.bg_fill=accent;
    v.widgets.inactive.rounding=egui::Rounding::same(9.0); v.widgets.hovered.rounding=egui::Rounding::same(9.0); v.window_rounding=egui::Rounding::same(18.0);
    ctx.set_visuals(v);
    let mut style=(*ctx.style()).clone();style.spacing.button_padding=egui::vec2(14.0,9.0);style.spacing.item_spacing=egui::vec2(10.0,10.0);ctx.set_style(style);
}
pub fn brand(ui: &mut egui::Ui) {
    let (r,_) = ui.allocate_exact_size(Vec2::splat(42.0),egui::Sense::hover());let p=ui.painter();
    p.rect_filled(r,11.0,Color32::from_rgb(36,53,47));
    let points=[(0.35,0.14),(0.65,0.14),(0.86,0.35),(0.86,0.65),(0.65,0.86),(0.35,0.86),(0.14,0.65),(0.14,0.35),(0.35,0.14)];
    let coords:Vec<Pos2>=points.iter().map(|(x,y)|r.min+egui::vec2(r.width()*x,r.height()*y)).collect();p.add(egui::Shape::line(coords,Stroke::new(2.0_f32,Color32::from_rgb(244,240,230))));
    let orange=Color32::from_rgb(229,107,63);let c=r.center();for (a,b) in [((-10.0,0.0),(-3.0,0.0)),((3.0,0.0),(10.0,0.0)),((0.0,-10.0),(0.0,-3.0)),((0.0,3.0),(0.0,10.0))] {p.line_segment([c+egui::vec2(a.0,a.1),c+egui::vec2(b.0,b.1)],Stroke::new(3.0_f32,orange));}p.circle_filled(c,3.4,orange);
}
