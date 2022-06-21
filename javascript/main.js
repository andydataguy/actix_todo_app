/**
 * This function takes a list of items from the Rust backend to render into an HTML div
 *
 * @param items {Array} - list of all to do items stored in the app
 * @param processType {String} - Defines what type of process that is going to be carried out (edit/delete)
 * @param elementId {String} - the id of the HTML element where the items will be inserted
 * @param processFunction {editItem | deleteItem} - Fires the described function when the button is clicked
 */

function renderItems(items, processType, elementId, processFunction) {

  // opens up an initial div block to contain the items
  let placeholder = "<div>"

  // temporary variable to hold each item as a dictionary of ID and title after being configured for HTML
  let itemsMeta = [];

  // Loops through each stored item and extracts the title 
  for (i = 0; i < items.length; i++) {
    let title = items[i]["title"];

    // Adds the process type (edit/delete) to the extracted title. 
    // Spaces are replaced with dashes to enable use in the HTML
    // example: placeholderId = "edit-to-do-item"
    let placeholderId = processType + "-" + title.replaceAll(" ", "-");

    // Creates HTML format for the item. Adds the placeholderId to a button and 
    // inserts text for the type of process to expect
    placeholder += '<div class="itemContainer">' 
                + '<p>'
                + title
                + '</p>' 
                + '<div class="actionButton" ' 
                + 'id="' 
                + placeholderId 
                + '">' 
                + processType 
                + '</div>' 
                + "</div>";

    // pushes the to do item into the itemsMeta array now configured for HTML
    itemsMeta.push({"id": placeholderId, "title": title});
  }

  // Once loop has finished, all items are defined with buttons and titles.
  // Closes the placeholder div block
  placeholder += "</div>"

  // inserts the placeholder into the HTML
  document.getElementById(elementId).innerHTML = placeholder;

  // loops through each item and defines an event listener on button click for each ID
  for (i = 0; i < itemsMeta.length; i++) {
    document.getElementById(itemsMeta[i]["id"])
            .addEventListener("click", processFunction);
  }
}

/**
 * This function interacts with our API methods to interact with the item list
 * 
 * @param {*} url {string} - the API endpoint to call
 * @param {*} method {string} - the HTTP method to use
 * @returns {XMLHttpRequest} - the request object that was made to the API
 */

function apiCall(url, method) {
  let xhr = new XMLHttpRequest();
  xhr.withCredentials = true;

  // Defines an event listener that renders to do items with JSON data for the API responses
  xhr.addEventListener('readystatechange', function() {
    if (this.readyState === this.DONE) {
      renderItems(JSON.parse(this.responseText)["pending_items"], "edit", "pendingItems", editItem);
      renderItems(JSON.parse(this.responseText)["done_items"], "delete", "doneItems", deleteItem);
      document.getElementById("completeNum").innerHTML = JSON.parse(this.responseText)["done_item_count"];
      document.getElementById("pendingNum").innerHTML = JSON.parse(this.responseText)["pending_item_count"];
    }
  });

  // Preps the API call object with the correct URL and method
  xhr.open(method, url);
  xhr.setRequestHeader('content-type', 'application/json');
  xhr.setRequestHeader('user-token', 'token'); // TODO: add proper authentication
  return xhr
}

/**
 * This function gets the title from `this` and calls the edit API end point
 */

function editItem() {
  let title = this.id.replaceAll("-", " ").replace("edit ", "");
  let call = apiCall("/item/edit", "PUT");
  let json = {
    "title": title, 
    "status": "done"
  };
  call.send(JSON.stringify(json));
}

/**
 * This function gets the title from `this` and calls the delete API end point
 */

function deleteItem() {
  let title = this.id.replaceAll("-", " ").replace("delete ", "");
  let call = apiCall("/item/delete", "POST");
  let json = {
    "title": title,
    "status": "done"
  };
  call.send(JSON.stringify(json));
}

/**
 * Loads the to do items to page by calling the `get` API endpoint
 */

function getItems() {
  let call = apiCall("/item/get", 'GET');
  call.send()
}
getItems();

// Adds a create button that calls the createItem function on click
document.getElementById("create-button").addEventListener("click", createItem);

/**
 * This function defines the create text input and button and calls the create API end point
 */
function createItem() {
  let title = document.getElementById("name");
  let call = apiCall("item/create/" + title.value, "POST");
  call.send();
  document.getElementById("name").value = null;
}