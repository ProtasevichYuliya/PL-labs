const fs = require("fs");
const https = require("https"); 

//функция, запрашивающая данные по протоколу https
//url - адрес, с которого нужно запрсить данные
//callback - функция, которая будет вызвана после успешного получения данных
function requestData(url, callback) {
    const request = https.get(url, function (message) {
        let response = "";

        //подписываемся на событие получения данных
        //"data" - это имя события, на которое мы подписываемся 
        message.on("data", function (data) {
            response += data;
        });
        
        //событие завершения передачи данных
        message.on("end", function () {
            const json = JSON.parse(response);
            callback(json);
        });
    });
    request.on("error", function (error) {
        console.error(error);
    });
}

function getHTML(events){
    let html = "";
    html += `<!DOCTYPE html>
    <html>
        <head>
            <meta charset="utf-8"/>
            <title>Upcoming events</title>
            <style>
                .numeration{
                    font-weight: bold; 
                }
                .numeration > ol{
                    font-weight: normal;
                }
            </style>
        </head>
        <body>
            <ol>`; 
    
    let dates = []; 
    for(let event of events){
        let date = event.local_date;
        let found_date = undefined;
        for(let i = 0; i < dates.length; i++){
            if (date == dates[i].date){
                found_date = dates[i];
                break;
            }
        }
        if (! found_date){
            found_date = {
                date:date,
                events: []
            };
            dates.push(found_date);
        }
        found_date.events.push(event);
    }
    for(let date of dates.sort((x,y)=>x.date > y.date)){
        html += `<li class="numeration">
            <p>${date.date}</p>
        <ol>`;
        for(let event of date.events){
            let location = "";
            if (event.venue) {
                location +=` ${event.venue.name}, ${event.venue.city}, ${event.venue.address_1}`;
            }
            else{
                location += "unknown";
            }
            html += `<li><p><strong>${event.name}</strong> will be on <strong>${event.local_date} at ${event.local_time}</strong>. Location: <strong>${location}</strong>. </p>
                ${event.description}
            </li>`;
        }
        html += `</ol>
            </li>`;
    }

    html += `</ol>
        </body>
    </html>`;
    return html;
}

let now = new Date();
let next_week = new Date();
next_week.setDate(now.getDate()+7);

requestData("https://api.meetup.com/2/cities?query=San+Antonio&country=us&state=tx", function (cities) {
    const city = cities.results.filter(x => x.city === "San Antonio")[0];
    console.log(`${city.city}: ${city.lat}, ${city.lon}`);
    requestData("https://api.meetup.com/find/topic_categories", function (topics){
        const topic = topics.filter(x => x.shortname === "tech")[0];
        console.log(`tech short name: ${topic.name}, ${topic.id}`);
        requestData(`https://api.meetup.com/find/upcoming_events?key=47203344f127143415d5716b2e4c42&lon=${city.lon}&lat=${city.lat}&topic_category=${topic.id}&start_date_range=${now.toISOString().substr(0,19)}&end_date_range=${next_week.toISOString().substr(0,19)}`, function(events){
            // for(let event of events.events){
            //     let location = "";
            //     if (event.venue) {
            //         location +=` ${event.venue.name}, ${event.venue.city}, ${event.venue.address_1}`;
            //     }
            //     else{
            //         location += "unknown";
            //     }
            //     console.log(`${event.name} \nwill be on ${event.local_date} at ${event.local_time}. \nLocation: ${location}. \nShort description: ${event.description} \n \n`);
            // }
            fs.writeFile("events.html", getHTML(events.events));
        });
    });
});