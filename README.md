# Openstock - Inventory management system

Openstock serves as a text project to learn api and 
application development in the rust programming language.
PostGreSQL will be used for storing data.

Openstock consists of two sub projects and a helper project:

## api
This project contains all the api calls needed to interact 
with the project.

## desktop
This project contains the desktop app written in iced.

## models
Models is the helper project which contains all models needed in 
both projects.

## Planned features:
- Login/register functionality (using session tokens)
- Customizable roles responsible for handling what users have access to

## Requirements
- Safe rust. Handle all errors!
- Secure credential handling

### Desktop
- Custom themes
- Components sized based on application size
- Home screen
    - Containing recent views and options to open available panels
- Based on tabs (like a web browser)
    - Changing the order of tabs
    - Splitscreen (two tabs beside each other)

