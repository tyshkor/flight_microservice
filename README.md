# flight_microservice

##API endpoints:

GET /path

Request of type application/json
Sample data:
```
{
    "flights": [["IND", "EWR"],["SFO", "ATL"],["GSO", "IND"],["ATL", "GSO"]]
}
```

Response
```
{
    "path": [
        "SFO",
        "EWR"
    ]
}
```