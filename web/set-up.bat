@echo off

echo cd /src
cd ..\src

@REM echo deleting /_app 
@REM rmdir /s /q _app
@REM echo /_app has been deleted!

echo deleting /templates
rmdir /s /q templates 
echo /templates has been deleted!

@REM echo deleting favicon.png
@REM del favicon.png
@REM echo favicon.png has been deleted!

echo back to /web
cd ..\web

echo starting run build scripts
npm run build & move templates ..\src\templates & cd ..\src\templates 
@REM & move _app ..\  & cd ..\..\app & copy static\favicon.png ..\src\ 
