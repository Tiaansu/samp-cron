# SampCRON

A cron plugin for samp/open.mp in Rust.

## Installation
- Download suitable binary files from releases for your operating system.
- Add it to your `plugins` folder
- Add `samp_cron` to server.cfg (config.json if you're using open.mp) or `samp_cron.so` (for linux)
- Add [samp_cron.inc](./include/samp_cron.inc) in includes folder

## Building
- Clone the repo
```bash
git clone https://github.com/Tiaansu/samp-cron.git
```
- Install Rust
```bash
rustup update stable-i686 --no-self-update && rustup default stable-i686
```
- Build using `cargo build`
```bash
cargo build --release
```

> [!IMPORTANT]
> Here's the scheduling format
> > You must follow it to work
> ```
> sec   min   hour   day of month   month   day of week   year
> *     *     *      *              *       *             *
> ```

## API
* #### cron_new(const pattern[], const callback[])
    * `pattern[]` - cron pattern
    * `callback[]` - callback to execute every call

    **Returns**   
        the cron job id

    **Example**   
    ```Pawn
    new CRON:cron_id = INVALID_CRON_ID;
    main()
    {
        cron_new("* * * * * *", "SecondTimer");
    }
    
    forward SecondTimer();
    public SecondTimer()
    {
        printf("Hi! I am called by cron id: %i", _:cron_id);
    }
    ```

* #### bool:cron_is_valid(CRON:id)
    * `id` - id of cron job

    **Returns**  
        true - valid  
        false - invalid  

    **Example**
    ```Pawn
    new CRON:cron_id = INVALID_CRON_ID;
    main()
    {
        printf("Is cron job id %i valid? %i", _:cron_id, cron_is_valid(cron_id));
    }
    ```

* #### bool:cron_delete(CRON:id)
    * `id` - id of cron job

    **Returns**   
        true - succeed  
        false - failed  

    **Example**
    ```Pawn
    new CRON:cron_id = INVALID_CRON_ID, current = 0;
    main()
    {
        cron_id = cron_new("* * * * * *", "SecondTimer");
    }

    forward SecondTimer();
    public SecondTimer()
    {
        if (++ current <= 10)
        {
            printf("%i/10", current + 1);
        }
        else
        {
            printf("Is cron job id %i deleted? %i", _:cron_id, cron_delete(cron_id));
        }
    }
    ```  

---

> [!NOTE]  
> Most of the part of this README is from/based on [samp-bcrypt](https://github.com/Sreyas-Sreelal/samp-bcrypt)
