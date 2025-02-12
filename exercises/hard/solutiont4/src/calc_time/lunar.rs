use crate::calc_time::Date;

const LUNAR_RANGE_SIZE: usize = (LUNAR_RANGE.end - LUNAR_RANGE.start) as usize;

pub struct LunarCalendar {
    lunar_dates: [(u8, u8); LUNAR_RANGE_SIZE],
}

impl LunarCalendar {
    pub(crate) fn new() -> Self {
        LunarCalendar {
            lunar_dates: LUNAR_NEW_YEAR_DATES,
        }
    }

    fn find_lunar_new_year(&self, year: i32) -> Option<Date> {
        if !LUNAR_RANGE.contains(&year) {
            return None;
        }

        let index = (year - LUNAR_RANGE.start) as usize;
        let (month, day) = self.lunar_dates[index];
        Date::new(year, month as u32, day as u32)
    }

    pub(crate) fn days_until_lunar_new_year(&self, date: &Date) -> Option<u32> {
        let current_new_year = self.find_lunar_new_year(date.year)?;

        if date.month > current_new_year.month
            || (date.month == current_new_year.month && date.day > current_new_year.day)
        {
            // 计算到下一个春节的天数
            let next_new_year = self.find_lunar_new_year(date.year + 1)?;
            let days_to_year_end = date.days_in_year() - date.days_from_year_start() + 1;
            let days_to_next_ny = next_new_year.days_from_year_start() - 1;
            Some(days_to_year_end + days_to_next_ny - 1)
        } else {
            // 计算到当年春节的天数
            let ny_days = current_new_year.days_from_year_start();
            let current_days = date.days_from_year_start();
            Some(ny_days - current_days - 1)
        }
    }
}

/// The lunar calendar is incalculable, the professional method is to make a mapping table.
/// See: https://www.zhihu.com/question/644610355/answer/3403827391

const LUNAR_RANGE: std::ops::Range<i32> = 1800..2101;
const LUNAR_NEW_YEAR_DATES: [(u8, u8); LUNAR_RANGE_SIZE] = [
    (1, 25), // 1800
    (2, 13), // 1801
    (2, 3),  // 1802
    (1, 23), // 1803
    (2, 11), // 1804
    (1, 31), // 1805
    (2, 18), // 1806
    (2, 7),  // 1807
    (1, 28), // 1808
    (2, 14), // 1809
    (2, 4),  // 1810
    (1, 25), // 1811
    (2, 13), // 1812
    (2, 1),  // 1813
    (1, 21), // 1814
    (2, 9),  // 1815
    (1, 29), // 1816
    (2, 16), // 1817
    (2, 5),  // 1818
    (1, 26), // 1819
    (2, 14), // 1820
    (2, 3),  // 1821
    (1, 23), // 1822
    (2, 11), // 1823
    (1, 31), // 1824
    (2, 18), // 1825
    (2, 7),  // 1826
    (1, 27), // 1827
    (2, 15), // 1828
    (2, 4),  // 1829
    (1, 25), // 1830
    (2, 13), // 1831
    (2, 2),  // 1832
    (2, 20), // 1833
    (2, 9),  // 1834
    (1, 29), // 1835
    (2, 17), // 1836
    (2, 5),  // 1837
    (1, 26), // 1838
    (2, 14), // 1839
    (2, 3),  // 1840
    (1, 23), // 1841
    (2, 10), // 1842
    (1, 30), // 1843
    (2, 18), // 1844
    (2, 7),  // 1845
    (1, 27), // 1846
    (2, 15), // 1847
    (2, 5),  // 1848
    (1, 24), // 1849
    (2, 12), // 1850
    (2, 1),  // 1851
    (2, 20), // 1852
    (2, 8),  // 1853
    (1, 29), // 1854
    (2, 17), // 1855
    (2, 6),  // 1856
    (1, 26), // 1857
    (2, 14), // 1858
    (2, 3),  // 1859
    (1, 23), // 1860
    (2, 10), // 1861
    (1, 30), // 1862
    (2, 18), // 1863
    (2, 8),  // 1864
    (1, 27), // 1865
    (2, 15), // 1866
    (2, 5),  // 1867
    (1, 25), // 1868
    (2, 11), // 1869
    (1, 31), // 1870
    (2, 19), // 1871
    (2, 9),  // 1872
    (1, 29), // 1873
    (2, 17), // 1874
    (2, 6),  // 1875
    (1, 26), // 1876
    (2, 13), // 1877
    (2, 2),  // 1878
    (1, 22), // 1879
    (2, 10), // 1880
    (1, 30), // 1881
    (2, 18), // 1882
    (2, 8),  // 1883
    (1, 28), // 1884
    (2, 15), // 1885
    (2, 4),  // 1886
    (1, 24), // 1887
    (2, 12), // 1888
    (1, 31), // 1889
    (1, 21), // 1890
    (2, 9),  // 1891
    (1, 30), // 1892
    (2, 17), // 1893
    (2, 6),  // 1894
    (1, 26), // 1895
    (2, 14), // 1896
    (2, 2),  // 1897
    (1, 22), // 1898
    (2, 10), // 1899
    (1, 31), // 1900
    (2, 19), // 1901
    (2, 8),  // 1902
    (1, 29), // 1903
    (2, 16), // 1904
    (2, 4),  // 1905
    (1, 25), // 1906
    (2, 13), // 1907
    (2, 2),  // 1908
    (1, 22), // 1909
    (2, 10), // 1910
    (1, 30), // 1911
    (2, 18), // 1912
    (2, 6),  // 1913
    (1, 26), // 1914
    (2, 14), // 1915
    (2, 3),  // 1916
    (1, 23), // 1917
    (2, 11), // 1918
    (2, 1),  // 1919
    (2, 20), // 1920
    (2, 8),  // 1921
    (1, 28), // 1922
    (2, 16), // 1923
    (2, 5),  // 1924
    (1, 24), // 1925
    (2, 13), // 1926
    (2, 2),  // 1927
    (1, 23), // 1928
    (2, 10), // 1929
    (1, 30), // 1930
    (2, 17), // 1931
    (2, 6),  // 1932
    (1, 26), // 1933
    (2, 14), // 1934
    (2, 4),  // 1935
    (1, 24), // 1936
    (2, 11), // 1937
    (1, 31), // 1938
    (2, 19), // 1939
    (2, 8),  // 1940
    (1, 27), // 1941
    (2, 15), // 1942
    (2, 5),  // 1943
    (1, 25), // 1944
    (2, 13), // 1945
    (2, 2),  // 1946
    (1, 22), // 1947
    (2, 10), // 1948
    (1, 29), // 1949
    (2, 17), // 1950
    (2, 6),  // 1951
    (1, 27), // 1952
    (2, 14), // 1953
    (2, 3),  // 1954
    (1, 24), // 1955
    (2, 12), // 1956
    (1, 31), // 1957
    (2, 18), // 1958
    (2, 8),  // 1959
    (1, 28), // 1960
    (2, 15), // 1961
    (2, 4),  // 1962
    (1, 25), // 1963
    (2, 13), // 1964
    (2, 2),  // 1965
    (1, 21), // 1966
    (2, 9),  // 1967
    (1, 30), // 1968
    (2, 17), // 1969
    (2, 6),  // 1970
    (1, 27), // 1971
    (2, 15), // 1972
    (2, 3),  // 1973
    (1, 23), // 1974
    (2, 11), // 1975
    (1, 31), // 1976
    (2, 18), // 1977
    (2, 7),  // 1978
    (1, 28), // 1979
    (2, 16), // 1980
    (2, 5),  // 1981
    (1, 25), // 1982
    (2, 13), // 1983
    (2, 2),  // 1984
    (2, 20), // 1985
    (2, 9),  // 1986
    (1, 29), // 1987
    (2, 17), // 1988
    (2, 6),  // 1989
    (1, 27), // 1990
    (2, 15), // 1991
    (2, 4),  // 1992
    (1, 23), // 1993
    (2, 10), // 1994
    (1, 31), // 1995
    (2, 19), // 1996
    (2, 7),  // 1997
    (1, 28), // 1998
    (2, 16), // 1999
    (2, 5),  // 2000
    (1, 24), // 2001
    (2, 12), // 2002
    (2, 1),  // 2003
    (1, 22), // 2004
    (2, 9),  // 2005
    (1, 29), // 2006
    (2, 18), // 2007
    (2, 7),  // 2008
    (1, 26), // 2009
    (2, 14), // 2010
    (2, 3),  // 2011
    (1, 23), // 2012
    (2, 10), // 2013
    (1, 31), // 2014
    (2, 19), // 2015
    (2, 8),  // 2016
    (1, 28), // 2017
    (2, 16), // 2018
    (2, 5),  // 2019
    (1, 25), // 2020
    (2, 12), // 2021
    (2, 1),  // 2022
    (1, 22), // 2023
    (2, 10), // 2024
    (1, 29), // 2025
    (2, 17), // 2026
    (2, 6),  // 2027
    (1, 26), // 2028
    (2, 13), // 2029
    (2, 3),  // 2030
    (1, 23), // 2031
    (2, 11), // 2032
    (1, 31), // 2033
    (2, 19), // 2034
    (2, 8),  // 2035
    (1, 28), // 2036
    (2, 15), // 2037
    (2, 4),  // 2038
    (1, 24), // 2039
    (2, 12), // 2040
    (2, 1),  // 2041
    (1, 22), // 2042
    (2, 10), // 2043
    (1, 30), // 2044
    (2, 17), // 2045
    (2, 6),  // 2046
    (1, 26), // 2047
    (2, 14), // 2048
    (2, 2),  // 2049
    (1, 23), // 2050
    (2, 11), // 2051
    (2, 1),  // 2052
    (2, 19), // 2053
    (2, 8),  // 2054
    (1, 28), // 2055
    (2, 15), // 2056
    (2, 4),  // 2057
    (1, 24), // 2058
    (2, 12), // 2059
    (2, 2),  // 2060
    (1, 21), // 2061
    (2, 9),  // 2062
    (1, 29), // 2063
    (2, 17), // 2064
    (2, 5),  // 2065
    (1, 26), // 2066
    (2, 14), // 2067
    (2, 3),  // 2068
    (1, 23), // 2069
    (2, 11), // 2070
    (1, 31), // 2071
    (2, 19), // 2072
    (2, 7),  // 2073
    (1, 27), // 2074
    (2, 15), // 2075
    (2, 5),  // 2076
    (1, 24), // 2077
    (2, 12), // 2078
    (2, 2),  // 2079
    (1, 22), // 2080
    (2, 9),  // 2081
    (1, 29), // 2082
    (2, 17), // 2083
    (2, 6),  // 2084
    (1, 26), // 2085
    (2, 14), // 2086
    (2, 3),  // 2087
    (1, 24), // 2088
    (2, 10), // 2089
    (1, 30), // 2090
    (2, 18), // 2091
    (2, 7),  // 2092
    (1, 27), // 2093
    (2, 15), // 2094
    (2, 5),  // 2095
    (1, 25), // 2096
    (2, 12), // 2097
    (2, 1),  // 2098
    (1, 21), // 2099
    (2, 9),  // 2100
];
