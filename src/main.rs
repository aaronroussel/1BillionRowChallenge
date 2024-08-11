

use std::fs::File;
use std::io::Write;
use rand::prelude::*;
use std::time::Instant;




fn main() -> std::io::Result<()> {
    // 200 cities
    let cities = vec![
        "Tokyo, Japan", "New York City, USA", "Paris, France", "Sydney, Australia", "Cairo, Egypt",
        "São Paulo, Brazil", "Toronto, Canada", "Istanbul, Turkey", "Berlin, Germany", "Bangkok, Thailand",
        "Nairobi, Kenya", "Mumbai, India", "Moscow, Russia", "Buenos Aires, Argentina", "Seoul, South Korea",
        "Johannesburg, South Africa", "Dubai, United Arab Emirates", "Madrid, Spain", "Singapore, Singapore",
        "Mexico City, Mexico", "London, United Kingdom", "Los Angeles, USA", "Rome, Italy", "Rio de Janeiro, Brazil",
        "Beijing, China", "Lagos, Nigeria", "Chicago, USA", "Hong Kong, China", "Tehran, Iran", "Lisbon, Portugal",
        "Vienna, Austria", "Zurich, Switzerland", "Jakarta, Indonesia", "Melbourne, Australia", "Copenhagen, Denmark",
        "Stockholm, Sweden", "Brussels, Belgium", "Warsaw, Poland", "Bangui, Central African Republic",
        "Oslo, Norway", "Helsinki, Finland", "Athens, Greece", "Bucharest, Romania", "Budapest, Hungary",
        "Prague, Czech Republic", "Bratislava, Slovakia", "Belgrade, Serbia", "Sofia, Bulgaria",
        "Tbilisi, Georgia", "Baku, Azerbaijan", "Yerevan, Armenia", "Kiev, Ukraine", "Minsk, Belarus",
        "Milan, Italy", "Barcelona, Spain", "Munich, Germany", "Hamburg, Germany", "Frankfurt, Germany",
        "Amsterdam, Netherlands", "Rotterdam, Netherlands", "Luxembourg City, Luxembourg", "Reykjavik, Iceland",
        "Valletta, Malta", "San Marino, San Marino", "Monaco, Monaco", "Andorra la Vella, Andorra",
        "Vatican City, Vatican City", "Kuala Lumpur, Malaysia", "Manila, Philippines", "Hanoi, Vietnam",
        "Ho Chi Minh City, Vietnam", "Phnom Penh, Cambodia", "Vientiane, Laos", "Bangui, Central African Republic",
        "Yaoundé, Cameroon", "Abuja, Nigeria", "Pretoria, South Africa", "Cape Town, South Africa",
        "Rabat, Morocco", "Algiers, Algeria", "Tunis, Tunisia", "Tripoli, Libya", "Nairobi, Kenya",
        "Addis Ababa, Ethiopia", "Kigali, Rwanda", "Bujumbura, Burundi", "Lusaka, Zambia",
        "Harare, Zimbabwe", "Maputo, Mozambique", "Luanda, Angola", "Windhoek, Namibia",
        "Gaborone, Botswana", "Antananarivo, Madagascar", "Victoria, Seychelles", "Port Louis, Mauritius",
        "Male, Maldives", "Colombo, Sri Lanka", "Kathmandu, Nepal", "Thimphu, Bhutan",
        "Dhaka, Bangladesh", "Islamabad, Pakistan", "Kabul, Afghanistan", "Baghdad, Iraq",
        "Damascus, Syria", "Amman, Jordan", "Jerusalem, Israel", "Beirut, Lebanon",
        "Doha, Qatar", "Manama, Bahrain", "Muscat, Oman", "Sana'a, Yemen", "Riyadh, Saudi Arabia",
        "Kuwait City, Kuwait", "Abu Dhabi, United Arab Emirates", "Dubai, United Arab Emirates",
        "Ankara, Turkey", "Athens, Greece", "Sofia, Bulgaria", "Belgrade, Serbia",
        "Tirana, Albania", "Skopje, North Macedonia", "Sarajevo, Bosnia and Herzegovina",
        "Zagreb, Croatia", "Ljubljana, Slovenia", "Podgorica, Montenegro", "Chisinau, Moldova",
        "Vilnius, Lithuania", "Riga, Latvia", "Tallinn, Estonia", "Moscow, Russia",
        "St. Petersburg, Russia", "Warsaw, Poland", "Prague, Czech Republic", "Bratislava, Slovakia",
        "Vienna, Austria", "Budapest, Hungary", "Bucharest, Romania", "Sofia, Bulgaria",
        "Belgrade, Serbia", "Skopje, North Macedonia", "Sarajevo, Bosnia and Herzegovina",
        "Zagreb, Croatia", "Ljubljana, Slovenia", "Podgorica, Montenegro", "Pristina, Kosovo",
        "San Salvador, El Salvador", "Guatemala City, Guatemala", "Managua, Nicaragua",
        "Tegucigalpa, Honduras", "Panama City, Panama", "San José, Costa Rica", "Havana, Cuba",
        "Santo Domingo, Dominican Republic", "Port-au-Prince, Haiti", "Kingston, Jamaica",
        "Nassau, Bahamas", "Bridgetown, Barbados", "Castries, Saint Lucia", "Kingstown, Saint Vincent and the Grenadines",
        "Saint George's, Grenada", "Port of Spain, Trinidad and Tobago", "Saint John's, Antigua and Barbuda",
        "Roseau, Dominica", "Basseterre, Saint Kitts and Nevis", "Belmopan, Belize",
        "Cayenne, French Guiana", "Georgetown, Guyana", "Paramaribo, Suriname", "Asunción, Paraguay",
        "Montevideo, Uruguay", "La Paz, Bolivia", "Sucre, Bolivia", "Quito, Ecuador",
        "Lima, Peru", "Caracas, Venezuela", "Bogotá, Colombia", "Brasília, Brazil",
        "Santiago, Chile", "Buenos Aires, Argentina", "Montevideo, Uruguay", "Asunción, Paraguay",
        "Suva, Fiji", "Apia, Samoa", "Nukuʻalofa, Tonga", "Port Moresby, Papua New Guinea",
        "Honiara, Solomon Islands", "Palikir, Micronesia", "Majuro, Marshall Islands",
        "Tarawa, Kiribati", "Funafuti, Tuvalu", "Port Vila, Vanuatu", "Nouméa, New Caledonia",
        "Papeete, French Polynesia", "Wellington, New Zealand", "Canberra, Australia"

    ];

    let mut file = File::create("foo.txt")?;

    let mut rng = rand::thread_rng();

    let start = Instant::now();

    for _n in 0..1000000000 {
        let temperature: f64 = rng.gen_range(5.0..42.00);
        let index: usize = rng.gen_range(1..195);
        let city = cities[index];
        let string = format!("{};{}\n", city, temperature);
        file.write_all(string.as_bytes())?;
    }

    let elapsed = start.elapsed();
    eprintln!("Elapsed: {}", elapsed.as_secs_f64());
    Ok(())
}



