// Extracts error messages from ORY UI nodes
// (Original ORY setup provides UI 'nodes' they want used)
// }
export function extractNestedErrorMessagesFromError(e) {
  const errs = []
  errs.push({ id: 0, type: 'error', text: JSON.stringify(e) })

  if (!e) return errs
  if (!('response' in e)) return errs
  if (!('data' in e.response)) return errs
  if ('error' in e.response.data) {
    // Non-UI (but still Ory-recognized) error returned, will have 'reason' field
    errs.push({ id: 0, type: 'error', text: e.response.data.error.reason })
  } else if ('ui' in e.response.data) {
    return extractNestedErrorMessagesFromUiData(e.response.data)
  } else {
    // Unknown error, just return it for debugging.
    // Ideally, this should never happen.
    errs.push({ id: 0, type: 'error', text: JSON.stringify(e.response.data) })
  }
  return errs
}
// e {
//      response: {
//          data : {
//              ui : {
//                  messages [ ... ]
//                  ...
//              }
//              messages: []
//              ...
//          }
//          ...
//      }
//  ...
// }
export function extractNestedErrorMessagesFromUiData(data) {
  let errs = []
  if ('messages' in data.ui) {
    errs = errs.concat(data.ui.messages)
  } else if ('nodes' in data.ui) {
    // sometimes, formatted slightly differently
    for (let i = 0; i < data.ui.nodes.length; i++) {
      const node = data.ui.nodes[i]
      errs = errs.concat(node.messages)
    }
  }
  return errs
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
export function extractNestedCsrfToken(data) {
  const returnedNodes = data.ui.nodes
  for (let i = 0; i < returnedNodes.length; i++) {
    if (returnedNodes[i].attributes.name === 'csrf_token') {
      return returnedNodes[i].attributes.value
    }
  }
  return '' // Return empty csrf token if it doesn't exist
}

// Extracts providers from ORY UI nodes
// labeled with 'provider' attribute
const preferredOrder = ['github', 'discord', 'google', 'apple', 'microsoft', 'gitlab']
export function extractOidcProviders(data) {
  const providers = []
  const returnedNodes = data.ui.nodes
  for (let i = 0; i < returnedNodes.length; i++) {
    if (returnedNodes[i].group === 'oidc' && returnedNodes[i].attributes.name === 'provider') {
      providers.push(returnedNodes[i].attributes.value)
    }
  }
  return providers.sort((a, b) => preferredOrder.indexOf(a) - preferredOrder.indexOf(b))
}
// labeled with 'link' attribute
export function extractOidcLinkProviders(data) {
  const providers = []
  const returnedNodes = data.ui.nodes
  for (let i = 0; i < returnedNodes.length; i++) {
    if (returnedNodes[i].group === 'oidc' && returnedNodes[i].attributes.name === 'link') {
      providers.push(returnedNodes[i].attributes.value)
    }
  }
  return providers.sort((a, b) => preferredOrder.indexOf(a) - preferredOrder.indexOf(b))
}
// labeled with 'unlink' attribute
export function extractOidcUnlinkProviders(data) {
  const providers = []
  const returnedNodes = data.ui.nodes
  for (let i = 0; i < returnedNodes.length; i++) {
    if (returnedNodes[i].group === 'oidc' && returnedNodes[i].attributes.name === 'unlink') {
      providers.push(returnedNodes[i].attributes.value)
    }
  }
  return providers.sort((a, b) => preferredOrder.indexOf(a) - preferredOrder.indexOf(b))
}

// Returns nested TOTP image and secret
// Return object:
// {
//    image: {
//        src: <base64 encoded image>
//        width: <width>
//        height: <height>
//    }
//    secret: <secret>
// }
export function extractNestedTotpData(data) {
  const returnedNodes = data.ui.nodes
  let image = null
  let secret = null
  for (let i = 0; i < returnedNodes.length; i++) {
    if (returnedNodes[i].group === 'totp') {
      if (returnedNodes[i].attributes.id === 'totp_qr') {
        image = {
          src: returnedNodes[i].attributes.src,
          width: returnedNodes[i].attributes.width,
          height: returnedNodes[i].attributes.height,
        }
      }
      if (returnedNodes[i].attributes.id === 'totp_secret_key') {
        secret = returnedNodes[i].attributes.text.text
      }
    }
  }
  return { image, secret }
}

// Returns nested lookup codes if they happen to be there
// {
//  enabled: <true/false> (whether lookup codes are currently enabled)
//  codes: [ <code>, <code>, ... ]
// }
export function extractNestedLookupCodes(data) {
  const returnedNodes = data.ui.nodes
  const codes = []
  let regenerateButton = false
  let disableButton = false

  for (let i = 0; i < returnedNodes.length; i++) {
    if (returnedNodes[i].group === 'lookup_secret') {
      if (returnedNodes[i].attributes.id === 'lookup_secret_codes') {
        // atributes.text.text returns comma separated list of codes, but we want them as an array
        // as we may want to format them differently
        for (const s of returnedNodes[i].attributes.text.context.secrets) {
          codes.push(s.text)
        }
      }
      if (returnedNodes[i].attributes.name === 'lookup_secret_regenerate') {
        regenerateButton = true
      }
      if (returnedNodes[i].attributes.name === 'lookup_secret_disable') {
        disableButton = true
      }
    }
  }
  return { codes, regenerateButton, disableButton }
}

export function getOryCookies() {
  const event = useRequestEvent()
  return process.server ? event.node.req.headers.cookie : document.cookie
}
