import { useParams } from 'react-router-dom';
import { useTranslation } from 'react-i18next';

export default function EditMonitor() {
  const { t } = useTranslation();
  const { id } = useParams();
  const pageName = id ? t('Edit Monitor') : t('Add New Monitor');

  return (
    <section className="edit-monitor-page" data-source="pages/EditMonitor.vue">
      <h1 className="mb-3">{pageName}</h1>
      <form>
        <div className="shadow-box shadow-box-with-fixed-bottom-bar edit-monitor-form">
          <div className="edit-monitor-row">
            <div className="edit-monitor-col">
              <h2 className="mb-2">{t('General')}</h2>

              <div className="my-3">
                <label htmlFor="type" className="form-label">{t('Monitor Type')}</label>
                <select id="type" className="form-select" data-testid="monitor-type-select" defaultValue="http">
                  <optgroup label={t('General Monitor Type')}>
                    <option value="http">HTTP(s)</option>
                    <option value="keyword">HTTP(s) - {t('Keyword')}</option>
                    <option value="port">TCP Port</option>
                    <option value="ping">Ping</option>
                    <option value="dns">DNS</option>
                    <option value="docker">{t('Docker Container')}</option>
                    <option value="real-browser">HTTP(s) - Browser Engine (Chrome/Chromium) (Beta)</option>
                  </optgroup>
                  <optgroup label={t('monitorTypeSpecial')}>
                    <option value="group">{t('Group')}</option>
                  </optgroup>
                  <optgroup label={t('Passive Monitor Type')}>
                    <option value="push">Push</option>
                    <option value="manual">Manual</option>
                  </optgroup>
                  <optgroup label={t('Specific Monitor Type')}>
                    <option value="globalping">Globalping - Access global monitoring probes</option>
                    <option value="grpc-keyword">gRPC(s) - {t('Keyword')}</option>
                    <option value="json-query">HTTP(s) - {t('Json Query')}</option>
                    <option value="mqtt">MQTT</option>
                    <option value="smtp">SMTP</option>
                    <option value="snmp">SNMP</option>
                    <option value="websocket-upgrade">Websocket Upgrade</option>
                  </optgroup>
                </select>
              </div>

              <div className="my-3">
                <label htmlFor="name" className="form-label">{t('Friendly Name')}</label>
                <input id="name" type="text" className="form-control" data-testid="friendly-name-input" placeholder={t('New Monitor')} />
              </div>

              <div className="my-3">
                <label htmlFor="url" className="form-label">{t('URL')}</label>
                <input id="url" type="url" className="form-control" placeholder="https://" />
              </div>

              <div className="my-3">
                <label htmlFor="interval" className="form-label">{t('Heartbeat Interval Detail')}</label>
                <input id="interval" type="number" className="form-control" defaultValue={60} min={1} max={2073600} step={1} />
                <div className="form-text">{t('1 minute')}</div>
              </div>

              <div className="my-3">
                <label htmlFor="maxRetries" className="form-label">{t('Retries')}</label>
                <input id="maxRetries" type="number" className="form-control" defaultValue={0} min={0} step={1} />
                <div className="form-text">{t('maxRetriesDescription')}</div>
              </div>

              <div className="my-3">
                <label htmlFor="timeout" className="form-label">{t('Request Timeout')} <span>{t('timeoutAfter', { seconds: 48 })}</span></label>
                <input id="timeout" type="number" className="form-control" defaultValue={48} min={0} step={0.1} />
              </div>

              <div className="my-3">
                <label htmlFor="resend-interval" className="form-label">{t('Resend Notification if Down X times')} <span>{t('resendDisabled')}</span></label>
                <input id="resend-interval" type="number" className="form-control" defaultValue={0} min={0} step={1} />
              </div>

              <h2 className="mt-5 mb-2">{t('Advanced')}</h2>
              <div className="my-3 form-check">
                <input id="expiry-notification" className="form-check-input" type="checkbox" />
                <label className="form-check-label" htmlFor="expiry-notification">{t('Certificate Expiry Notification')}</label>
                <div className="form-text">{t('advanceConfigInSettings')}</div>
              </div>
              <div className="my-3 form-check">
                <input id="domain-expiry-notification" className="form-check-input" type="checkbox" defaultChecked />
                <label className="form-check-label" htmlFor="domain-expiry-notification">Domain Name Expiry Notification</label>
                <div className="form-text">{t('advanceConfigInSettings')}</div>
              </div>
              <div className="my-3 form-check">
                <input id="ignore-tls" className="form-check-input" type="checkbox" />
                <label className="form-check-label" htmlFor="ignore-tls">{t('ignoreTLSError')}</label>
              </div>
              <div className="my-3 form-check">
                <input id="upside-down" className="form-check-input" type="checkbox" />
                <label className="form-check-label" htmlFor="upside-down">{t('Upside Down Mode')}</label>
                <div className="form-text">{t('upsideDownModeDescription')}</div>
              </div>

              <div className="my-3">
                <label htmlFor="maxRedirects" className="form-label">{t('Max Redirects')}</label>
                <input id="maxRedirects" type="number" className="form-control" defaultValue={10} min={0} step={1} />
                <div className="form-text">{t('maxRedirectDescription')}</div>
              </div>

              <div className="my-3 form-check">
                <input id="saveErrorResponse" className="form-check-input" type="checkbox" defaultChecked />
                <label className="form-check-label" htmlFor="saveErrorResponse">{t('Save HTTP Error Response')}</label>
                <div className="form-text">{t('saveResponseDescription')}</div>
              </div>
              <div className="my-3 form-check">
                <input id="saveResponse" className="form-check-input" type="checkbox" />
                <label className="form-check-label" htmlFor="saveResponse">{t('Save HTTP Success Response')}</label>
                <div className="form-text">{t('saveResponseDescription')}</div>
              </div>

              <div className="my-3">
                <label htmlFor="responseMaxLength" className="form-label">{t('Response Max Length')}</label>
                <input id="responseMaxLength" type="number" className="form-control" defaultValue={1024} min={0} step={1} />
                <div className="form-text">{t('responseMaxLengthDescription')}</div>
              </div>

              <div className="my-3">
                <label htmlFor="acceptedStatusCodes" className="form-label">{t('Accepted Status Codes')}</label>
                <input id="acceptedStatusCodes" className="form-control" defaultValue="200-299" />
                <div className="form-text">{t('acceptedStatusCodesDescription')}</div>
              </div>
            </div>

            <div className="edit-monitor-col">
              <h2 className="mb-2">{t('Notifications')}</h2>
              <p>{t('Not available, please setup.')}</p>
              <button type="button" className="btn btn-primary me-2">{t('Setup Notification')}</button>

              <h2 className="mt-5 mb-2">Proxy</h2>
              <p>{t('Not available, please setup.')}</p>
              <button type="button" className="btn btn-primary me-2">{t('Setup Proxy')}</button>

              <h2 className="mt-5 mb-2">{t('HTTP Options')}</h2>
              <div className="my-3">
                <label htmlFor="method" className="form-label">{t('Method')}</label>
                <select id="method" className="form-select" defaultValue="GET">
                  <option value="GET">GET</option>
                  <option value="POST">POST</option>
                  <option value="PUT">PUT</option>
                  <option value="PATCH">PATCH</option>
                  <option value="DELETE">DELETE</option>
                  <option value="HEAD">HEAD</option>
                  <option value="OPTIONS">OPTIONS</option>
                </select>
              </div>

              <div className="my-3">
                <label htmlFor="httpBodyEncoding" className="form-label">{t('Body Encoding')}</label>
                <select id="httpBodyEncoding" className="form-select" defaultValue="json">
                  <option value="json">JSON</option>
                  <option value="form">x-www-form-urlencoded</option>
                  <option value="xml">XML</option>
                </select>
              </div>

              <div className="my-3">
                <label htmlFor="body" className="form-label">{t('Body')}</label>
                <textarea id="body" className="form-control code-textarea" placeholder={'範例：\n{\n    "key": "value"\n}'} />
              </div>

              <div className="my-3">
                <label htmlFor="headers" className="form-label">{t('Headers')}</label>
                <textarea id="headers" className="form-control code-textarea" placeholder={'範例：\n{\n    "HeaderName": "HeaderValue"\n}'} />
              </div>

              <h2 className="mt-5 mb-2">{t('Authentication')}</h2>
              <div className="my-3">
                <label htmlFor="authMethod" className="form-label">{t('Method')}</label>
                <select id="authMethod" className="form-select" defaultValue="none">
                  <option value="none">{t('None')}</option>
                  <option value="basic">Basic</option>
                  <option value="ntlm">NTLM</option>
                  <option value="oauth2-cc">OAuth2: Client Credentials</option>
                </select>
              </div>
            </div>
          </div>

          <div className="fixed-bottom-bar">
            <button type="submit" className="btn btn-primary">{t('Save')}</button>
          </div>
        </div>
      </form>
    </section>
  );
}
