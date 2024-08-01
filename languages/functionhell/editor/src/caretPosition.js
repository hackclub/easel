
function getCaretPosition(element) {
    const { selectionStart } = element;
    const { left, top } = getCaretCoordinates(element, selectionStart);
    const { left: elementLeft, top: elementTop } = element.getBoundingClientRect();
    return {
        left: elementLeft + left,
        top: elementTop + top
    };
}

function getCaretCoordinates(element, position) {
    const div = document.createElement('div');
    const style = getComputedStyle(element);
    for (const prop of style) {
        div.style[prop] = style[prop];
    }
    div.style.position = 'absolute';
    div.style.whiteSpace = 'pre-wrap';
    div.style.visibility = 'hidden';

    const textContent = element.value.substring(0, position);
    const textNode = document.createTextNode(textContent);
    div.appendChild(textNode);

    document.body.appendChild(div);
    const span = document.createElement('span');
    span.textContent = element.value.substring(position) || '.';
    div.appendChild(span);

    const { offsetLeft: left, offsetTop: top } = span;
    document.body.removeChild(div);
    return { left, top };
}