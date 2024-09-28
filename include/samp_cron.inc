#if defined _inc_samp_cron
    #undef _inc_samp_cron
#endif

#if defined _samp_rcon_included
    #endinput
#endif
#define _samp_rcon_included

#define INVALID_CRON_ID         (CRON:0)

/*
cron_new(const pattern[], const callback[], const args[] = "", {Float, _}:...)
    Params
        `pattern[]` - cron pattern
        `callback[]` - callback to execute every call
        `args[]` - custom arguments

    Returns
        the cron job id

    Example
        ```
        new CRON:cron_id = INVALID_CRON_ID;
        main()
        {
            cron_new("* * * * * *", "SecondTimer", "i", 5);
        }
        
        forward SecondTimer(num);
        public SecondTimer(num)
        {
            printf("Hi! I am called by cron id: %i and I also have a custom args: %i!", _:cron_id, num);
        }
        ```
 */
native CRON:cron_new(const pattern[], const callback[], const args[] = "", {Float, _}:...);

/*
bool:cron_is_valid(CRON:id)
    Params
        `id` - id of cron job

    Returns
        true - valid
        false - invalid

    Example
        ```
        new CRON:cron_id = INVALID_CRON_ID;
        main()
        {
            printf("Is cron job id %i valid? %i", _:cron_id, cron_is_valid(cron_id));
        }
        ```
 */
native bool:cron_is_valid(CRON:id);

/*
bool:cron_delete(CRON:id)
    Params
        `id` - id of cron job

    Returns
        true - succeed
        false - failed

    Example
        ```
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
 */
native bool:cron_delete(CRON:id);