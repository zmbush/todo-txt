fn is_line_ending(chr: char) -> bool
{
    chr == '\n' || chr == '\r'
}

named!(date<&str, ::Date>,
    do_parse!(
        year:
            take!(4) >>
            tag_s!("-") >>
        month:
            take!(2) >>
            tag_s!("-") >>
        day:
            take!(2) >>
            tag_s!(" ") >>
        (
            ::Date::from_ymd(
                year.parse().unwrap(),
                month.parse().unwrap(),
                day.parse().unwrap()
            )
        )
    )
);

named!(priority<&str, u8>,
    do_parse!(
            tag_s!("(") >>
        priority:
            take!(1) >>
            tag_s!(") ") >>
        (
            priority.as_bytes()[0] - b'A'
        )
    )
);

named!(parse<&str, ::Task>,
    do_parse!(
        finished:
            opt!(tag_s!("x ")) >>
        priority:
            opt!(priority) >>
        finish_date:
            opt!(date) >>
        create_date:
            opt!(date) >>
        subject:
            take_till!(is_line_ending) >>
        ({
            ::Task {
                subject: subject.to_owned(),
                priority: if priority.is_none() {
                    26
                } else {
                    priority.unwrap()
                },
                create_date: if create_date.is_none() {
                    finish_date
                } else {
                    create_date
                },
                finish_date: if create_date.is_none() {
                    None
                } else {
                    finish_date
                },
                finished: finished.is_some(),
            }
        })
    )
);

pub fn task(line: &String) -> ::Task
{
    parse(line)
        .unwrap()
        .1
}
