// import other modules
pub mod main {
    fn main() {
        /*
        kernel_terminal: File = FileManager::getDevice("terminal");
        keyboard input: File = FileManager::getDevice("keyboard");
        let error: u8 = 'main_loop: loop {
            *do stuff*
            match result {
                Result::Continue => continue;
                Rusult::Shutdown => break 0;
                Result::Exception => {
                    success: u8 = recover(&result);
                    if success == 0 {
                        continue 'main_loop;
                    }
                    else if success == 1 {
                        break 'main_loop success;
                    }
                }
                Result::Error(error_code) => break error_code;
            }
        }
        match error {
            0 => {
                kernel_terminal.print("Shutdown successfully")  
            }
            err => {
                kernel_terminal.print("Samsara ran into an error. Exit code {err}.");
            }
        }
        */
    }

    //typedef
    trait Process {
        // add code here
    }
    trait Schedular {
        // add code here
    }
    
    trait File {
        // add code here
    }
    trait FileManager {
        // add code here
    }
    trait Page {
        // add code here
    }
    trait MemoryManager {
        // add code here
    }
    trait Module {
        // add code here
        fn start();
        fn stop();
        fn restart();
    }
    trait Server {
        // add code here
    }
    trait Translator {
        // add code here }
    
}

