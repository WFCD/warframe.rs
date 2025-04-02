export def languages []: nothing -> list<string> {
    return [
        "ko"
        "ru"
        "de"
        "fr"
        "pt"
        "zh-hans"
        "zh-hant"
        "es"
        "it"
        "pl"
        "uk"
        "en"
    ]
}

# Returns the response of a GET request to the specified endpoint
export def market_req [
    endpoint:string, # The endpoint to request
    language?:string@languages # The language to request the endpoint in
]: nothing -> table {
    if language == null {
        return (http get $"https://api.warframe.market/v2($endpoint)?language=en")
    } else {
        return (http get $"https://api.warframe.market/v2($endpoint)" --headers [Language $language])
    }
}

# Returns the response of a GET request to the specified endpoint
export def worldstate_req [
    endpoint:string, # The endpoint to request
    language?:string # The language to request the endpoint in
]: nothing -> table {
    if language == null {
        return (http get $"https://api.warframestat.us/pc($endpoint)")
    } else {
        return (http get $"https://api.warframestat.us/pc($endpoint)?language=($language)")
    }
}