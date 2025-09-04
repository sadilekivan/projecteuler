def epoch_to_date(epoch_seconds):
    SECONDS_PER_DAY = 86400
    days = epoch_seconds // SECONDS_PER_DAY

    # Constants
    DAYS_PER_400_YEARS = 146097
    DAYS_PER_100_YEARS = 36524
    DAYS_PER_4_YEARS = 1461
    DAYS_PER_YEAR = 365

    # 400-year blocks
    n400 = days // DAYS_PER_400_YEARS
    days %= DAYS_PER_400_YEARS

    # 100-year blocks (max 3)
    n100 = min(days // DAYS_PER_100_YEARS, 3)
    days -= n100 * DAYS_PER_100_YEARS

    # 4-year blocks
    n4 = days // DAYS_PER_4_YEARS
    days %= DAYS_PER_4_YEARS

    # 1-year blocks (max 3)
    n1 = min(days // DAYS_PER_YEAR, 3)
    days -= n1 * DAYS_PER_YEAR

    year = 400 * n400 + 100 * n100 + 4 * n4 + n1
    print(n400, n100, n4, n1)
    # Day of year to month/day
    is_leap = year % 4 == 0 and (year % 100 != 0 or year % 400 == 0)
    normal_cum = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365]
    leap_cum = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366]
    cum = leap_cum if is_leap else normal_cum

    def day_of_year_to_month_day(d, c):
        if d < c[1]:
            return 1, d + 1
        if d < c[2]:
            return 2, d - c[1] + 1
        if d < c[3]:
            return 3, d - c[2] + 1
        if d < c[4]:
            return 4, d - c[3] + 1
        if d < c[5]:
            return 5, d - c[4] + 1
        if d < c[6]:
            return 6, d - c[5] + 1
        if d < c[7]:
            return 7, d - c[6] + 1
        if d < c[8]:
            return 8, d - c[7] + 1
        if d < c[9]:
            return 9, d - c[8] + 1
        if d < c[10]:
            return 10, d - c[9] + 1
        if d < c[11]:
            return 11, d - c[10] + 1
        return 12, d - c[11] + 1

    month, day = day_of_year_to_month_day(days, cum)
    return day, month, year


if __name__ == "__main__":
    from datetime import datetime, UTC

    # Test range: some key timestamps
    test_epochs = [
        0,  # 0000-01-01
        86400 * 365,  # 0001-01-01 (non-leap year)
        86400 * 366,  # 0001-01-02 (leap year adjustment)
        1609459200,  # 2021-01-01
        1580515200,  # 2020-02-01 (leap year)
        1230768000,  # 2009-01-01
        31556995200,  # year ~1000
        63113990400,  # year ~2000
    ]

    for epoch in range(0, int(10e10)):
        epoch *= 86400
        d, m, y = epoch_to_date(epoch)
        dt = datetime.fromtimestamp(epoch, UTC)
        y += 1970
        assert y == dt.year, f"{dt}, {y}"
        # assert (d, m, y) == (dt.day, dt.month, dt.year), (
        #     f"Mismatch on epoch {epoch}: got {(d, m, y)}, expected {(dt.day, dt.month, dt.year)}"
        # )

    print("All tests passed.")
