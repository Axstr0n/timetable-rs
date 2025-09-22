use bimap::BiMap;
use chrono::NaiveTime;
use once_cell::sync::Lazy;

fn t(h: u32, m: u32) -> NaiveTime {
    NaiveTime::from_hms_opt(h, m, 0).unwrap()
}

pub static START_TIME_MAP: Lazy<BiMap<&'static str, NaiveTime>> = Lazy::new(|| {
    let mut map = BiMap::new();
    map.insert("0.00%", t(7, 0));
    map.insert("3.85%", t(7, 30));
    map.insert("7.69%", t(8, 0));
    map.insert("11.54%", t(8, 30));
    map.insert("15.38%", t(9, 0));
    map.insert("19.23%", t(9, 30));
    map.insert("23.08%", t(10, 0));
    map.insert("26.92%", t(10, 30));
    map.insert("30.77%", t(11, 0));
    map.insert("34.62%", t(11, 30));
    map.insert("38.46%", t(12, 0));
    map.insert("42.31%", t(12, 30));
    map.insert("46.15%", t(13, 0));
    map.insert("50.00%", t(13, 30));
    map.insert("53.85%", t(14, 0));
    map.insert("57.69%", t(14, 30));
    map.insert("61.54%", t(15, 0));
    map.insert("65.38%", t(15, 30));
    map.insert("69.23%", t(16, 0));
    map.insert("73.08%", t(16, 30));
    map.insert("76.92%", t(17, 0));
    map.insert("80.77%", t(17, 30));
    map.insert("84.62%", t(18, 0));
    map.insert("88.47%", t(18, 30));
    map.insert("88.47%", t(18, 30));
    map.insert("92.32%", t(19, 0));
    map.insert("96.17%", t(19, 30));
    map.insert("100.00%", t(20, 0));
    map
});

pub static DURATION_MAP: Lazy<BiMap<&'static str, NaiveTime>> = Lazy::new(|| {
    let mut map = BiMap::new();
    map.insert("3.85%", t(0, 30));
    map.insert("7.69%", t(1, 0));
    map.insert("11.54%", t(1, 30));
    map.insert("15.38%", t(2, 0));
    map.insert("19.23%", t(2, 30));
    map.insert("23.08%", t(3, 0));
    map.insert("26.92%", t(3, 30));
    map.insert("30.77%", t(4, 0));
    map.insert("34.62%", t(4, 30));
    map.insert("38.46%", t(5, 0));
    map.insert("42.31%", t(5, 30));
    map.insert("46.15%", t(6, 0));
    map.insert("50.00%", t(6, 30));
    map.insert("53.85%", t(7, 0));
    map
});
