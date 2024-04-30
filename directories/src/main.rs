use directories::{BaseDirs, UserDirs,ProjectDirs};

fn main() {
    if let Some(proj_dirs)=ProjectDirs::from("com", "Foo Corp","Bar App"){      //I can not understand.
        println!("{:?}",proj_dirs.config_dir());
        println!("{:?}",proj_dirs.data_local_dir());
        println!("{:?}",proj_dirs.cache_dir());
    }
    println!();
    if let Some(base_dirs)=BaseDirs::new(){
        println!("{:?}",base_dirs.home_dir());
        println!("{:?}",base_dirs.cache_dir());
        println!("{:?}",base_dirs.config_dir());
        println!("{:?}",base_dirs.config_local_dir());      //与cache_dir()相同
        println!("{:?}",base_dirs.data_dir());              //与cache_dir()相同
        println!("{:?}",base_dirs.data_local_dir());        //与cache_dir()相同
        println!("{:?}",base_dirs.preference_dir());
        
    }
    println!();
    if let Some(user_dirs)=UserDirs::new(){      
        println!("{:?}",user_dirs.home_dir());  
        println!("{:?}",user_dirs.audio_dir());
        println!("{:?}",user_dirs.desktop_dir());
        println!("{:?}",user_dirs.document_dir());
        println!("{:?}",user_dirs.download_dir());
        println!("{:?}",user_dirs.picture_dir());
        println!("{:?}",user_dirs.public_dir());
        println!("{:?}",user_dirs.video_dir());
        println!("{:?}",user_dirs.font_dir());        
    }
    println!();
    if let Some(go)=ProjectDirs::from("com","apple","user"){
        println!("{:?}",go.config_dir())
    }
}
