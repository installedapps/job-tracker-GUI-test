use crate::{Stats, Status};

pub struct SankeyGraph {
    total: usize,
    flows: [(Status, usize); 4],
}

impl SankeyGraph {
    pub fn from_stats(stats: &Stats) -> Self {
        Self {
            total: stats.total,
            flows: Status::all().map(|status| (status, stats.count_for(status))),
        }
    }

    pub fn to_svg(&self) -> String {
        let mut svg = String::from(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="920" height="420" viewBox="0 0 920 420">
<rect width="920" height="420" rx="18" fill="#07101f"/>
<text x="40" y="46" fill="#e2e8f0" font-family="Inter, Arial, sans-serif" font-size="24" font-weight="700">Job Pipeline Sankey</text>
"##,
        );

        svg.push_str(&format!(
            r##"<rect x="48" y="166" width="190" height="88" rx="12" fill="#0b1830" stroke="#2563eb"/>
<text x="72" y="202" fill="#bfdbfe" font-family="Inter, Arial, sans-serif" font-size="18" font-weight="700">Applications: {}</text>
"##,
            self.total
        ));

        if self.total == 0 {
            svg.push_str(
                r##"<text x="72" y="230" fill="#94a3b8" font-family="Inter, Arial, sans-serif" font-size="14">No applications yet</text>
</svg>"##,
            );
            return svg;
        }

        for (index, (status, count)) in self.flows.iter().enumerate() {
            let y = 88 + index as i32 * 78;
            let width = 22 + ((*count as f32 / self.total as f32) * 72.0) as i32;
            let colours = status.colours();

            svg.push_str(&format!(
                r##"<path d="M 238 210 C 360 210, 382 {mid}, 512 {mid}" fill="none" stroke="{stroke}" stroke-width="{width}" stroke-linecap="round" opacity="0.78"/>
<rect x="548" y="{box_y}" width="250" height="52" rx="10" fill="{background}" stroke="{stroke}"/>
<text x="568" y="{text_y}" fill="{text}" font-family="Inter, Arial, sans-serif" font-size="16" font-weight="700">{label}: {count}</text>
"##,
                mid = y + 26,
                box_y = y,
                text_y = y + 32,
                stroke = colours.border,
                background = colours.background,
                text = colours.text,
                label = status,
                count = count,
                width = width,
            ));
        }

        svg.push_str("</svg>");
        svg
    }
}
