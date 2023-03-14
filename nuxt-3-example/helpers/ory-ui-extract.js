// Extracts error messages from ORY UI nodes  
// (Original ORY setup provides UI 'nodes' they want used)
// Input 'e' is the raw error, and returned 'errs' is an concatenated array of each 'messages' in:
// e {
//      response: {
//          data : {
//              ui : {
//                  messages [ ... ]
//                  ...
//              }
//              ...
//          }
//          ...    
//      }
//  ...
// } 
export function extract_nested_error_messages_from_error(e) {
    let errs = []
    if ("data" in e.response)
        if ("messages" in e.response.data.ui) {
            errs = errs.concat(e.response.data.ui.messages)
        } else if ("nodes" in e.response.data.ui) {
            // sometimes, formatted slightly differently
            for(let i = 0; i < e.response.data.ui.nodes.length; i ++){
                let node = e.response.data.ui.nodes[i];
                errs = errs.concat(node.messages)
        }
    }
    return errs;
}

// Extracts csrf_token from ORY UI nodes  
// (Original ORY setup provides UI 'nodes' they want used)
// Input data is the 'data' subfield of a flow data (such as from ory.getVerificationFlow())
// Returned csrf_token is nested 'value' field of attribute matching 'csrf_token' as shown below:
//    data : {
//        ui : {
//            nodes {
//                attributes {
//                   name: "csrf_token"
//                   value: <this value>
//                }
//            }
//            messages [ ... ]
//            ...
//        }
//        ...
//    }
export function extract_nested_csrf_token(data) {
    let returned_nodes = data.ui.nodes;
    for (let i = 0; i < returned_nodes.length; i++){
        if (returned_nodes[i].attributes.name=="csrf_token"){
            return returned_nodes[i].attributes.value;
        }
    }
    return ''; // Return empty csrf token if it doesn't exist
}
