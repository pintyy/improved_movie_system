use std::fmt;

#[derive(Debug)]
struct Movie {
    name:String,
    publish_year:u32,
    famous_actors:String,
    movie_info:String,
}
#[derive(Debug)]

struct Movie_database {
    movies:Vec<Movie>
}

impl Movie {
    fn new (name:String,publish_year:u32, famous_actors:String,movie_info:String) -> Self {

        Movie { 
           
            
            name,
            publish_year,
            famous_actors,
            movie_info,

        }

        
    }

    
}

impl Movie_database {
    fn new () -> Self 
    {
        Movie_database {movies:vec![]}

    }

    fn add_movie (&mut self, movie:Movie) {
        self.movies.push(movie);
    }

    fn movie_find_by_name (&self, name:&str) -> Option<&Movie>{
        self.movies.iter().find(|movie|movie.name==name)

    }
    fn movie_find_by_year(&self, year: u32) -> Vec<&Movie> {
    self.movies.iter().filter(|movie| movie.publish_year <= year).collect() }
}


fn main () {
    let mut movie_database=Movie_database::new();
    let movie1 = Movie::new(
        String::from("Dexter"),
        2006,
        String::from("Michael C. Hall, Jennifer Carpenter, Julie Benz"),
        String::from("Dexter is a crime drama series that follows Dexter Morgan, a blood-spatter analyst for the Miami Metro Police Department. By day, Dexter works as a forensic expert, but by night, he becomes a vigilante serial killer who only targets other killers. Haunted by a dark past, Dexter struggles with his own moral code, all while hiding his secret from his sister, friends, and colleagues.")
    );
    
    let movie2 = Movie::new(
        String::from("Breaking Bad"),
        2008,
        String::from("Bryan Cranston, Anna Gunn, Aaron Paul"),
        String::from("Breaking Bad follows Walter White, a high school chemistry teacher diagnosed with terminal cancer who turns to cooking methamphetamine to secure his family's financial future. Partnering with former student Jesse Pinkman, Walter transforms from a mild-mannered teacher into the ruthless drug lord 'Heisenberg.' The series explores themes of morality, crime, and the unintended consequences of one's choices.")
    );
    
    let movie3 = Movie::new(
        String::from("Game of Thrones"),
        2011,
        String::from("Emilia Clarke, Kit Harington, Peter Dinklage"),
        String::from("Game of Thrones is an epic fantasy series set in the fictional continents of Westeros and Essos. Various noble families, including the Starks, Lannisters, and Targaryens, vie for control of the Iron Throne, all while an ancient enemy, the White Walkers, threatens to return and bring winter upon the realm. The series is known for its complex characters, political intrigue, and intense battles.")
    );
    
    let movie4 = Movie::new(
        String::from("The Sopranos"),
        1999,
        String::from("James Gandolfini, Lorraine Bracco, Edie Falco"),
        String::from("The Sopranos follows mob boss Tony Soprano, who tries to balance the demands of his family life with his role as the leader of a New Jersey crime family. Suffering from panic attacks, Tony seeks therapy, revealing the inner turmoil of his violent and morally ambiguous life. The series explores themes of loyalty, mental health, and the dark side of the American Dream.")
    );
    
    let movie5 = Movie::new(
        String::from("Stranger Things"),
        2016,
        String::from("Millie Bobby Brown, David Harbour, Winona Ryder"),
        String::from("Stranger Things is set in the 1980s in the small town of Hawkins, Indiana, where a young boy's mysterious disappearance uncovers a government conspiracy involving a parallel dimension called the Upside Down. As his friends search for him, they meet Eleven, a girl with telekinetic powers, who may hold the key to finding him. The show combines supernatural horror, nostalgia, and friendship.")
    );
    
    let movie6 = Movie::new(
        String::from("The Wire"),
        2002,
        String::from("Dominic West, Idris Elba, Michael K. Williams"),
        String::from("The Wire is a gritty, realistic portrayal of life in Baltimore, Maryland. Each season focuses on a different facet of the city, from the drug trade and police force to the school system and political landscape. Through the lives of cops, dealers, and residents, the series explores the systemic failures that affect urban life and challenge notions of justice.")
    );
    
    let movie7 = Movie::new(
        String::from("Sherlock"),
        2010,
        String::from("Benedict Cumberbatch, Martin Freeman"),
        String::from("Sherlock is a modern adaptation of Sir Arthur Conan Doyle's Sherlock Holmes, set in contemporary London. Consulting detective Sherlock Holmes and his friend Dr. John Watson solve complex cases, often facing off against his nemesis, Moriarty. Known for his sharp intellect and unorthodox methods, Sherlock's keen observation skills help him solve crimes that baffle the police.")
    );
    
    let movie8 = Movie::new(
        String::from("The Office"),
        2005,
        String::from("Steve Carell, Rainn Wilson, John Krasinski"),
        String::from("The Office is a mockumentary-style sitcom that follows the everyday lives of employees at the Dunder Mifflin paper company in Scranton, Pennsylvania. Led by bumbling manager Michael Scott, the quirky staff face absurd challenges and awkward interactions. The show combines humor and heartfelt moments, exploring the ups and downs of workplace relationships.")
);

let movie9 = Movie::new(
    String::from("Friends"),
    1994,
    String::from("Jennifer Aniston, Courteney Cox, Matthew Perry"),
    String::from("Friends is a sitcom that follows six friends – Rachel, Monica, Phoebe, Joey, Chandler, and Ross – as they navigate life, love, and careers in New York City. With humor and heart, the series captures the struggles and triumphs of early adulthood, solidifying itself as a cultural touchstone of the '90s.")
);

let movie10 = Movie::new(
    String::from("Black Mirror"),
    2011,
    String::from("Bryce Dallas Howard, Daniel Kaluuya, Jon Hamm"),
    String::from("Black Mirror is an anthology series that explores the dark side of technology and its impact on society. Each episode is a standalone story set in a dystopian world where advancements in AI, social media, and surveillance often lead to unexpected and disturbing outcomes. The series critiques modern society's dependence on technology.")
);

let movie11 = Movie::new(
    String::from("House of Cards"),
    2013,
    String::from("Kevin Spacey, Robin Wright"),
    String::from("House of Cards follows Congressman Frank Underwood and his wife, Claire, as they manipulate their way to power in Washington, D.C. The series offers a look into the dark world of American politics, exploring themes of ambition, betrayal, and the corruption that often lies beneath the surface of political power.")
);

let movie12 = Movie::new(
    String::from("True Detective"),
    2014,
    String::from("Matthew McConaughey, Woody Harrelson"),
    String::from("True Detective is an anthology crime series where each season follows a new case and set of detectives. The first season follows Rust Cohle and Marty Hart as they investigate a series of brutal murders over several decades, unraveling secrets that challenge their beliefs and morals.")
);

let movie13 = Movie::new(
    String::from("Better Call Saul"),
    2015,
    String::from("Bob Odenkirk, Rhea Seehorn, Jonathan Banks"),
    String::from("Better Call Saul is the prequel to Breaking Bad, exploring the backstory of small-time lawyer Jimmy McGill, who eventually becomes the morally dubious Saul Goodman. The series delves into Jimmy's struggles and transformation, examining his relationships and his complex path toward a life of crime.")
);

let movie14 = Movie::new(
    String::from("The Mandalorian"),
    2019,
    String::from("Pedro Pascal, Gina Carano"),
    String::from("The Mandalorian is set in the Star Wars universe, following a lone bounty hunter known as the Mandalorian. As he travels the galaxy taking on dangerous assignments, he encounters 'The Child' (Grogu), a mysterious and powerful infant of Yoda's species. Their bond transforms his life, leading to a journey of honor and redemption.")
);

let movie15 = Movie::new(
    String::from("Westworld"),
    2016,
    String::from("Evan Rachel Wood, Thandiwe Newton, Jeffrey Wright"),
    String::from("Westworld is a science fiction series set in a futuristic amusement park populated by advanced robots called hosts. As guests interact with these lifelike robots, questions of consciousness, free will, and morality arise. The series explores humanity's relationship with technology, power, and the blurred lines between reality and simulation.")
);
   
   
   
    movie_database.add_movie(movie1);
    movie_database.add_movie(movie2);
    movie_database.add_movie(movie3);
    movie_database.add_movie(movie4);
    movie_database.add_movie(movie5);
    movie_database.add_movie(movie6);
    movie_database.add_movie(movie7);
    movie_database.add_movie(movie8);
    movie_database.add_movie(movie9);
    movie_database.add_movie(movie10);
    movie_database.add_movie(movie11);
    movie_database.add_movie(movie12);
    movie_database.add_movie(movie13);
    movie_database.add_movie(movie14);
    movie_database.add_movie(movie15);




    let movie_name="Better Call Saul";

    if let Some(movie) = movie_database.movie_find_by_name(movie_name) {
        println!("All informations about {} : 
        
Publish year: {} 

Famous actors : {}

About movie : {}"
,movie.name,movie.publish_year, movie.famous_actors, movie.movie_info)
}
    else {
    println!("Movie '{}' not found in the database.", movie_name);
    }
    
   
    println!("----------------------");
    


    let before_year=2016;
    let movies_before_2016=movie_database.movie_find_by_year(before_year);
    println!("Movies before {}:", before_year);
    for movie in movies_before_2016  {
        println!("{:#?}",movie.name)
        
    }
    

    //println!("{:#?}",movie_database); tüm databasei yazdırıyor
}

