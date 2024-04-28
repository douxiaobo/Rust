fn main() {

    if let Some(home_dir) = dirs::home_dir() {
        println!("home_dir:{}", home_dir.display());
    } else {
        println!("Home directory not found.");
    }

    if let Some(cache_dir)=dirs::cache_dir(){
        println!("cache_dir:{}",cache_dir.display());
    }else{
        println!("Cache directory not found.");
    }

    if let Some(config_dir) = dirs::config_dir() {
        println!("config_dir:{}", config_dir.display());
    } else {
        println!("Config directory not found.");
    }    

    if let Some(config_local_dir)=dirs::config_local_dir(){
        println!("config_local_dir:{}",config_local_dir.display());
    }else{
        println!("Config local directory not found.");
    }

    if let Some(data_dir) = dirs::data_dir() {  //相当于dirs::config_dir()
        println!("data_dir:{}", data_dir.display());
    } else {
        println!("Data directory not found.");
    }

    if let Some(data_local_dir) = dirs::data_local_dir() {  //相当于dirs::data_dir()
        println!("data_local_dir:{}", data_local_dir.display());
    } else {
        println!("Data local directory not found.");
    }

    if let Some(executable_dir) = dirs::executable_dir() {      //Mac OS可能不支持
        println!("executable_dir:{}", executable_dir.display());
    } else {
        println!("Executable directory not found.");
    }

    if let Some(preference_dir)=dirs::preference_dir(){
        println!("preference_dir:{}",preference_dir.display());
    }else{
        println!("Preference directory not found.");
    }

    if let Some(runtime_dir) = dirs::runtime_dir() {      //Mac OS可能不支持
        println!("runtime_dir:{}", runtime_dir.display());
    } else {
        println!("Runtime directory not found.");
    }

    if let Some(state_dir)=dirs::state_dir(){
        println!("state_dir:{}",state_dir.display());
    }else{
        println!("State directory not found.");
    }

    if let Some(audio_dir) = dirs::audio_dir() {
        println!("audio_dir:{}", audio_dir.display());
    } else {
        println!("Audio directory not found.");
    }

    if let Some(desktop_dir) = dirs::desktop_dir() {
        println!("desktop_dir:{}", desktop_dir.display());
    } else {
        println!("Desktop directory not found.");
    }

    if let Some(document_dir)=dirs::document_dir(){
        println!("document_dir:{}",document_dir.display());
    }else{
        println!("Documents directory not found.");
    }

    if let Some(download_dir) = dirs::download_dir() {
        println!("download__dir:{}", download_dir.display());
    } else {
        println!("Download directory not found.");
    }

    if let Some(font_dir)=dirs::font_dir(){
        println!("font_dir:{}",font_dir.display());
    }else{
        println!("Font directory not found.");
    } 

    if let Some(picture_dir) = dirs::picture_dir() {
        println!("picture_dir:{}", picture_dir.display());
    } else {
        println!("Picture directory not found.");
    }
    
    if let Some(public_dir) = dirs::public_dir() {
        println!("public_dir:{}", public_dir.display());
    } else {
        println!("Public directory not found.");
    }

    if let Some(template_dir) = dirs::template_dir() {      //Mac OS可能不支持
        println!("template_dir:{}", template_dir.display());
    } else {
        println!("Template directory not found.");
    }
    
    if let Some(video_dir) = dirs::video_dir() {
        println!("video_dir:{}", video_dir.display());
    } else {
        println!("Video directory not found.");
    }
}
