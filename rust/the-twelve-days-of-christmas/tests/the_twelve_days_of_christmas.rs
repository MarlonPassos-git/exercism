use ::the_twelve_days_of_christmas::the_twelve_days_of_chrisms;

#[test]
fn test_generate_twelve_days_of_chrisms() {
    let expected_song = "On the first day of Christmas,\nmy true love sent to me\nA partridge in a pear tree.\n\n\
                             On the second day of Christmas,\nmy true love sent to me\nTwo turtle doves,\n\
                             And a partridge in a pear tree.\n\n\
                             On the third day of Christmas,\nmy true love sent to me\nThree French hens,\n\
                             Two turtle doves,\nAnd a partridge in a pear tree.\n\n\
                             On the fourth day of Christmas,\nmy true love sent to me\nFour calling birds,\n\
                             Three French hens,\nTwo turtle doves,\nAnd a partridge in a pear tree.\n\n\
                             On the fifth day of Christmas,\nmy true love sent to me\nFive golden rings,\n\
                             Four calling birds,\nThree French hens,\nTwo turtle doves,\nAnd a partridge in a pear tree.\n\n\
                             On the sixth day of Christmas,\nmy true love sent to me\nSix geese a-laying,\n\
                             Five golden rings,\nFour calling birds,\nThree French hens,\nTwo turtle doves,\n\
                             And a partridge in a pear tree.\n\n\
                             On the seventh day of Christmas,\nmy true love sent to me\nSeven swans a-swimming,\n\
                             Six geese a-laying,\nFive golden rings,\nFour calling birds,\nThree French hens,\n\
                             Two turtle doves,\nAnd a partridge in a pear tree.\n\n\
                             On the eighth day of Christmas,\nmy true love sent to me\nEight maids a-milking,\n\
                             Seven swans a-swimming,\nSix geese a-laying,\nFive golden rings,\nFour calling birds,\n\
                             Three French hens,\nTwo turtle doves,\nAnd a partridge in a pear tree.\n\n\
                             On the ninth day of Christmas,\nmy true love sent to me\nNine ladies dancing,\n\
                             Eight maids a-milking,\nSeven swans a-swimming,\nSix geese a-laying,\nFive golden rings,\n\
                             Four calling birds,\nThree French hens,\nTwo turtle doves,\nAnd a partridge in a pear tree.\n\n\
                             On the tenth day of Christmas,\nmy true love sent to me\nTen lords a-leaping,\n\
                             Nine ladies dancing,\nEight maids a-milking,\nSeven swans a-swimming,\nSix geese a-laying,\n\
                             Five golden rings,\nFour calling birds,\nThree French hens,\nTwo turtle doves,\n\
                             And a partridge in a pear tree.\n\n\
                             On the eleventh day of Christmas,\nmy true love sent to me\nEleven pipers piping,\n\
                             Ten lords a-leaping,\nNine ladies dancing,\nEight maids a-milking,\nSeven swans a-swimming,\n\
                             Six geese a-laying,\nFive golden rings,\nFour calling birds,\nThree French hens,\n\
                             Two turtle doves,\nAnd a partridge in a pear tree.\n\n\
                             On the twelfth day of Christmas,\nmy true love sent to me\nTwelve drummers drumming,\n\
                             Eleven pipers piping,\nTen lords a-leaping,\nNine ladies dancing,\nEight maids a-milking,\n\
                             Seven swans a-swimming,\nSix geese a-laying,\nFive golden rings,\nFour calling birds,\n\
                             Three French hens,\nTwo turtle doves,\nAnd a partridge in a pear tree!\n";

    assert_eq!(the_twelve_days_of_chrisms(), expected_song);
}
