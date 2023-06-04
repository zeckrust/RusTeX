use std::collections::LinkedList;


pub trait Item
{
}

pub struct Package
{
    pub name: String,
    pub options: LinkedList<String>
}

pub struct Table
{

}

pub struct Figure
{

}

pub struct Enumerate
{

}

pub struct Section
{
    name: String,
    display_num: bool
}

pub struct SubSection
{
    name: String,
    display_num: bool
}

pub struct SubSubSection
{
    name: String,
    display_num: bool
}