// GoatCounter: https://www.goatcounter.com
// This file is released under the ISC license: https://opensource.org/licenses/ISC
;(function() {
    'use strict';

    window.goatcounter = window.goatcounter || {};

    function endpoint() {
        var script = document.querySelector('script[data-goatcounter]');
        return script ? script.dataset.goatcounter : '';
    }

    function canCount() {
        return !!endpoint();
    }

    window.goatcounter.count = function() {
        if (!canCount()) {
            return;
        }

        var img = document.createElement('img');
        img.src = endpoint() + '?p=' + encodeURIComponent(location.pathname);
        img.alt = '';
        img.setAttribute('aria-hidden', 'true');
        img.style.position = 'absolute';
        img.style.width = '1px';
        img.style.height = '1px';
        document.body.appendChild(img);
    };
})();
